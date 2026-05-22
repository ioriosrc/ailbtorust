```rust
use serde::{Deserialize, Serialize};
use std::io;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

#[derive(Serialize, Deserialize)]
pub struct LoadedExtension {
    buffer: Option<Vec<u8>>,
    raw: String,
}

pub enum TypeExtensionLoader {
    Browser,
    Server,
    Filesystem,
}

#[derive(Serialize, Deserialize)]
pub struct InstallExtensionProps {
    foxe_file_data: Vec<u8>,
    file: Option<File>,
    external_id: Option<String>,
}

/**
 * An extension loader is an object used by studio to list, install, and uninstall extensions
 * from a particular namespace.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct IExtensionLoader {
    namespace: String,
    type: TypeExtensionLoader,

    // get extension by id
    async fn get_extension(&self, id: &str) -> Result<Option<ExtensionInfo>, io::Error>;

    // get a list of installed extensions
    async fn get_extensions(&self) -> Result<Vec<ExtensionInfo>, io::Error>;

    // load the source code for a specific extension
    async fn load_extension(&self, id: &str) -> Result<LoadedExtension, io::Error>;

    // install extension contained within the file data
    async fn install_extension(&self, props: InstallExtensionProps) -> Result<ExtensionInfo, io::Error>;

    // uninstall extension with id
    async fn uninstall_extension(&self, id: &str) -> Result<(), io::Error>;
}
```

Note: The Rust code above uses the `serde` crate for serialization and deserialization, which is not directly available in JavaScript. You may need to handle file uploads differently in Rust compared to JavaScript. Additionally, the `async` keywords are used for asynchronous operations, which might be necessary depending on the context of your application.