```rust
use std::fs::{self, DirEntry, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use serde_json::{self, Value};
use jszip::ZipFile;

/// Represents a loaded extension.
pub struct LoadedExtension {
    pub raw: Vec<u8>,
}

/// Represents an extension package.json file.
#[derive(Debug, Deserialize)]
pub struct ExtensionPackageJson {
    name: String,
    version: String,
    publisher: Option<String>,
}

/// Manages the installation and uninstallation of extensions.
pub struct ExtensionsHandler {
    user_extensions_dir: PathBuf,
}

impl ExtensionsHandler {
    pub fn new(user_dir: PathBuf) -> Self {
        Self {
            user_extensions_dir,
        }
    }

    async fn safe_read_file(path: &Path) -> String {
        match fs::read_to_string(path) {
            Ok(data) => data,
            Err(_) => "".to_string(),
        }
    }

    /// Returns a unique identifier for an extension based on the publisher and package name.
    pub async fn get_package_id(pkg_json: Option<&ExtensionPackageJson>) -> String {
        if let Some(pkg_json) = pkg_json {
            let { name, version } = &pkg_json;
            if !name.trim().is_empty() && !version.trim().is_empty() {
                return format!("{}.{}", name.replace(' ', "-").replace('/', "-"), version);
            }
        }
        panic!("Missing package.json");
    }

    /// Get the directory name to use for an installed extension.
    pub fn get_package_dirname(pkg_json: &ExtensionPackageJson) -> String {
        let pkg_id = ExtensionsHandler::get_package_id(Some(pkg_json)).to_string();
        if pkg_id.len() >= 255 {
            panic!("package.json publisher.name-version is too long");
        }
        format!("{}.{}", pkg_id, pkg_json.version)
    }

    /// Separate a package.json "name" field into separate namespace (i.e. @foxglove) and name fields.
    pub fn parse_package_name(name: &str) -> PackageName {
        let res = /^@([^/]+)\/(.+)/.exec(name);
        if res.is_some() {
            return PackageName {
                namespace: res[1].to_string(),
                name: res[2],
            };
        }
        PackageName { name }
    }

    pub async fn get(id: &str) -> Option<DesktopExtension> {
        let extension_base_dir = self.user_extensions_dir.join(id);
        if !extension_base_dir.exists() {
            return None;
        }

        let package_path = extension_base_dir.join("package.json");
        let package_data = fs::read_to_string(package_path).await.unwrap();
        let package_json: ExtensionPackageJson = serde_json::from_str(&package_data).unwrap();

        let readme_path = extension_base_dir.join("README.md");
        let changelog_path = extension_base_dir.join("CHANGELOG.md");

        let readme = ExtensionsHandler::safe_read_file(&readme_path).await;
        let changelog = ExtensionsHandler::safe_read_file(&changelog_path).await;

        Some(DesktopExtension {
            id: package_id.to_string(),
            package_json,
            directory: extension_base_dir.clone(),
            readme,
            changelog,
        })
    }

    pub async fn list() -> Vec<DesktopExtension> {
        let extensions = Vec::new();
        if !self.user_extensions_dir.exists() {
            return extensions;
        }

        for entry in fs::read_dir(self.user_extensions_dir).await.unwrap() {
            let path = entry.unwrap().path();

            if path.is_dir() {
                let package_path = path.join("package.json");
                let package_data = fs::read_to_string(package_path).await.unwrap();
                let package_json: ExtensionPackageJson = serde_json::from_str(&package_data).unwrap();

                let readme_path = path.join("README.md");
                let changelog_path = path.join("CHANGELOG.md");

                let readme = ExtensionsHandler::safe_read_file(&readme_path).await;
                let changelog = ExtensionsHandler::safe_read_file(&changelog_path).await;

                extensions.push(DesktopExtension {
                    id: ExtensionsHandler::get_package_id(Some(&package_json)).to_string(),
                    package_json,
                    directory: path.clone(),
                    readme,
                    changelog,
                });
            }
        }

        extensions
    }

    pub async fn load(id: &str) -> Result<LoadedExtension, Box<dyn std::error::Error>> {
        let extension = self.get(id).await?;
        let source_path = extension.directory.join(&extension.package_json.main);
        let raw = fs::read_to_string(source_path).await?;
        Ok(LoadedExtension { raw })
    }

    pub async fn install(foxe_file_data: Vec<u8>) -> Result<DesktopExtension, Box<dyn std::error::Error>> {
        // Open the archive
        let mut archive = ZipFile::new(&foxe_file_data);

        // Check for a package.json file
        if !archive.contains("package.json") {
            return Err("Extension does not contain a package.json file");
        }

        // Unpack and parse the package.json file
        let pkg_json_str = archive.read_entry("package.json").await?;
        let pkg_json: ExtensionPackageJson = serde_json::from_str(&pkg_json_str).unwrap();

        let readme_str = archive.read_entry("README.md").await?;
        let changelog_str = archive.read_entry("CHANGELOG.md").await?;

        // Check for basic validity of package.json and get the packageId
        let pkg_id = ExtensionsHandler::get_package_id(Some(&pkg_json)).to_string();
        if pkg_id.len() >= 255 {
            return Err("package.json publisher.name-version is too long");
        }
        let dir = ExtensionsHandler::get_package_dirname(&pkg_json);

        // Delete any previous installation and create the extension folder
        let extension_base_dir = self.user_extensions_dir.join(dir);
        if extension_base_dir.exists() {
            fs::remove_dir_all(&extension_base_dir).await?;
        }
        fs::create_dir_all(&extension_base_dir).await?;

        // Unpack all files into the extension folder
        for (entry, file) in archive.entries().await? {
            let path = entry.path();
            if path.is_dir() {
                fs::create_dir_all(path).await?;
            } else {
                let mut file_data = Vec::new();
                file.read_to_end(&mut file_data)?;
                fs::write(path, &file_data).await?;
            }
        }

        Ok(DesktopExtension {
            id: pkg_id.to_string(),
            package_json,
            directory: extension_base_dir.clone(),
            readme: readme_str,
            changelog: changelog_str,
        })
    }

    pub async fn uninstall(id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let extension = self.get(id).await?;
        fs::remove_dir_all(extension.directory.clone()).await?;
        Ok(())
    }
}

/// Represents a desktop extension.
pub struct DesktopExtension {
    id: String,
    package_json: ExtensionPackageJson,
    directory: PathBuf,
    readme: String,
    changelog: String,
}
```