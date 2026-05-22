```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::fs;
use std::io::{Read, Write};

async fn extract_target_architecture(executable_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let arch = match executor::get_target_architecture() {
        x if x == "arm64" => "arm64",
        x if x == "x86_64" => "x86_64",
        x if x == "universal" => "universal",
        _ => return Err(Box::from("Unsupported arch")),
    };
    if arch != "universal" {
        let output = executor::exec_command(
            &format!("lipo -extract {arch} {executable_path} -output {executable_path}"),
        )?;
        log_info!(target_arch: arch, "Extracted {}", target_arch);
    }
    Ok(())
}

async fn copy_spotlight_importer(context: crate::core::Context) -> Result<(), Box<dyn std::error::Error>> {
    let { electron_platform_name, out_dir, app_out_dir } = context;
    if electron_platform_name != "darwin" {
        return Ok(());
    }

    let appName = context.packager.app_info.product_filename;
    let app_path = path.join(app_out_dir, format!("{}.app", appName));
    let spotlight_dir_path = path.join(app_path, "Contents", "Library", "Spotlight");
    let zip_path = path.join(out_dir, "MCAPSpotlightImporter.mdimporter.zip");
    fs::remove_file(zip_path)?;

    let zip_url =
        "https://github.com/foxglove/MCAPSpotlightImporter/releases/download/v1.0.2/MCAPSpotlightImporter.mdimporter.zip";
    let zip_sha = "26cafa3e3069fcbd294864ceeee1bc9899e94456e0d28079f966787b6f05c7a2";
    executor::download_file(zip_url, zip_path)?;
    let actual_sha = crypto::digest::sha256.digest(&fs::read_all(zip_path)?);
    if actual_sha != zip_sha {
        return Err(Box::from("SHA mismatch for {}", zip_url));
    }
    executor::extract_zip(zip_path, spotlight_dir_path)?;

    let executable_path = path.join(
        spotlight_dir_path,
        "MCAPSpotlightImporter.mdimporter",
        "Contents",
        "MacOS",
        "MCAPSpotlightImporter",
    );
    executor::extract_target_architecture(executable_path)?;

    // The notarization step requires a valid signature from our "Developer ID Application"
    // certificate. However this certificate is only available in CI, so for packaging to succeed in a
    // local development workflow, we just use the "-" ad-hoc signing identity.
    //
    // electron-builder's MacPackager creates a temporary keychain to hold the signing info. The
    // certificate is not in the regular system keychain so we have to use the temporary keychain for
    // signing.
    let keychain_file = executor::get_keychain_file(context)?;
    if keychain_file.is_some() {
        let output = executor::exec_command(
            &format!(
                "security find-identity -v -p codesigning {keychain_file}",
            ),
        )?;
        log_info!(target_arch: "adhoc", "Found keychain {}", target_arch);
    }

    let signing_args =
        if executor::is_ci() && keychain_file.is_some() {
            &["--keychain", keychain_file.unwrap(), "--sign", "Developer ID Application"]
        } else {
            &["--sign", "-"];
        };

    let output = executor::exec_command(&format!(
        "{} --force --options runtime --entitlements {} {}",
        signing_args.join(" "),
        path::join(crate::core::context::resolve_path("quicklookjs/index.d.ts"), "../dist/PreviewExtension.entitlements"),
        executable_path,
    ))?;

    log_info!("Re-sandboxed appex");

    Ok(())
}

async fn configure_quick_look_extension(context: crate::core::Context) -> Result<(), Box<dyn std::error::Error>> {
    let { electron_platform_name, app_out_dir } = context;
    if electron_platform_name != "darwin" {
        return Ok(());
    }

    let appName = context.packager.app_info.product_filename;
    let app_path = path.join(app_out_dir, format!("{}.app", appName));
    let app_bundle_id = context.packager.config.app_id;

    let appex_path = path.join(app_path, "Contents", "PlugIns", "PreviewExtension.appex");
    let appex_contents = path.join(appex_path, "Contents");
    let appex_resources = path.join(appex_contents, "Resources");
    let appex_info_plist = path.join(appex_contents, "Info.plist");
    let appex_executable_path = path.join(appex_contents, "MacOS", "PreviewExtension");

    let original_info = plist::parse(
        &fs::read_to_string(appex_info_plist)?,
    )?;
    let new_info = {
        let mut new_info = original_info.clone();
        new_info.cfp_bundle_identifier = Some(format!("{}.quicklook", app_bundle_id));
        new_info
            .ns_extension()
            .map(|mut ns_extension| {
                ns_extension.ns_extension_attributes_mut().unwrap().ql_supported_content_types.push("org.ros.bag".to_string());
                ns_extension.ns_extension_attributes_mut().unwrap().ql_supports_searchable_items = false;
                ns_extension
            })
            .unwrap();
        new_info.qljs_mut().map(|mut qljs| {
            qljs.page_path_mut().unwrap().push_str("index.html");
            qljs
        })
        .unwrap();
        new_info
    };
    let plist_data = plist::build(&new_info);
    fs::write(appex_info_plist, plist_data)?;
    log_info!("Updated appex Info.plist for Quick Look");

    let webpack_output_dir = path::join(context.packager.info.app_dir, "quicklook");
    for entry in fs::read_dir(webpack_output_dir)? {
        if let Ok(entry) = entry {
            if !entry.file_type()?.is_file() {
                return Err(Box::from("Expected only files in Quick Look webpack output, found: {}", entry.name()));
            }
            fs::copy(&entry.path(), path::join(appex_resources, entry.name()))?;
        }
    }

    executor::extract_target_architecture(appex_executable_path)?;

    let keychain_file = executor::get_keychain_file(context)?;
    if let Some(keychain_file) = keychain_file {
        let output = executor::exec_command(
            &format!("security find-identity -v -p codesigning {keychain_file}"),
        )?;
        log_info!(target_arch: "adhoc", "Found keychain {}", target_arch);
    }

    let signing_args =
        if executor::is_ci() && keychain_file.is_some() {
            &["--keychain", keychain_file.unwrap(), "--sign", "Developer ID Application"]
        } else {
            &["--sign", "-"];
        };

    let output = executor::exec_command(&format!(
        "{} --force --options runtime --entitlements {} {}",
        signing_args.join(" "),
        path::join(crate::core::context::resolve_path("quicklookjs/index.d.ts"), "../dist/PreviewExtension.entitlements"),
        appex_executable_path,
    ))?;

    log_info!("Re-sandboxed appex");

    Ok(())
}
```