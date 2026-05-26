// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Extension loader: parses .foxe ZIP archives using pure Rust (zip crate).
//! Zero JavaScript — decompression runs at native WASM speed.

use std::io::{Cursor, Read};

use super::types::{ExtensionFormat, ExtensionInfo, FoxeContents, PackageJson, StoredExtension};

/// Parse a .foxe file (ZIP archive) from raw bytes using pure Rust.
/// Only extracts the files we need: package.json, dist/extension.js, README.md, CHANGELOG.md.
pub fn parse_foxe(data: &[u8]) -> Result<FoxeContents, String> {
    let reader = Cursor::new(data);
    let mut archive = zip::ZipArchive::new(reader)
        .map_err(|e| format!("Invalid ZIP archive: {}", e))?;

    let mut package_json: Option<String> = None;
    let mut extension_js: Option<String> = None;
    let mut readme = String::new();
    let mut changelog = String::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("ZIP entry error: {}", e))?;

        let name = file.name().to_string();

        match name.as_str() {
            "package.json" => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)
                    .map_err(|e| format!("Failed to read package.json: {}", e))?;
                package_json = Some(buf);
            }
            "dist/extension.js" => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)
                    .map_err(|e| format!("Failed to read dist/extension.js: {}", e))?;
                extension_js = Some(buf);
            }
            "README.md" | "readme.md" => {
                file.read_to_string(&mut readme).ok();
            }
            "CHANGELOG.md" | "changelog.md" => {
                file.read_to_string(&mut changelog).ok();
            }
            _ => {
                // Skip all other files (lazy: don't decompress what we don't need)
            }
        }
    }

    Ok(FoxeContents {
        package_json: package_json
            .ok_or_else(|| "Missing package.json in .foxe archive".to_string())?,
        extension_js: extension_js
            .ok_or_else(|| "Missing dist/extension.js in .foxe archive".to_string())?,
        readme,
        changelog,
    })
}

/// Validate and parse the package.json manifest, returning ExtensionInfo.
pub fn parse_manifest(foxe: &FoxeContents, archive_size: usize) -> Result<ExtensionInfo, String> {
    let pkg: PackageJson = serde_json::from_str(&foxe.package_json)
        .map_err(|e| format!("Invalid package.json: {}", e))?;

    if pkg.name.is_empty() {
        return Err("Invalid extension: missing name".to_string());
    }

    let (publisher, name) = parse_package_name(&pkg.name, &pkg.publisher)?;

    let display_name = if pkg.display_name.is_empty() {
        name.clone()
    } else {
        pkg.display_name.clone()
    };

    let normalized_publisher = publisher
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == ' ' || *c == '-')
        .collect::<String>();

    let id = format!(
        "{}.{}",
        normalized_publisher.replace(' ', "").replace('-', ""),
        name
    );

    Ok(ExtensionInfo {
        id,
        name,
        publisher: normalized_publisher,
        display_name,
        version: pkg.version,
        description: pkg.description,
        license: pkg.license,
        homepage: pkg.homepage,
        keywords: pkg.keywords,
        readme: foxe.readme.clone(),
        changelog: foxe.changelog.clone(),
        size: archive_size,
        format: ExtensionFormat::Legacy,
    })
}

/// Create a StoredExtension from raw .foxe bytes.
/// Extracts once and stores the JS source directly — no base64, no re-decompression.
pub fn load_foxe_extension(data: &[u8]) -> Result<StoredExtension, String> {
    let foxe = parse_foxe(data)?;
    let info = parse_manifest(&foxe, data.len())?;

    Ok(StoredExtension {
        info,
        extension_js: foxe.extension_js,
    })
}

/// Get the extension JS source from a StoredExtension.
/// Simply returns the pre-extracted source — zero overhead.
pub fn get_extension_source(stored: &StoredExtension) -> Result<String, String> {
    Ok(stored.extension_js.clone())
}

/// Parse publisher and name from a package name string.
fn parse_package_name(raw_name: &str, explicit_publisher: &str) -> Result<(String, String), String> {
    if let Some(stripped) = raw_name.strip_prefix('@') {
        if let Some((scope, name)) = stripped.split_once('/') {
            let publisher = if explicit_publisher.is_empty() {
                scope.to_string()
            } else {
                explicit_publisher.to_string()
            };
            return Ok((publisher, name.to_string()));
        }
    }

    if raw_name.contains('.') {
        if let Some((pub_part, name_part)) = raw_name.split_once('.') {
            let publisher = if explicit_publisher.is_empty() {
                pub_part.to_string()
            } else {
                explicit_publisher.to_string()
            };
            return Ok((publisher, name_part.to_string()));
        }
    }

    if explicit_publisher.is_empty() {
        return Err("Invalid extension: missing publisher".to_string());
    }
    Ok((explicit_publisher.to_string(), raw_name.to_string()))
}
