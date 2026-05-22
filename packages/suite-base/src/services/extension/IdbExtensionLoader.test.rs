```rust
use std::fs;
use jszip::JsZip;

use idb::IndexedDb;
use serde_json::{self, Value};

use crate::{
    storage::ExtensionInfo,
    types::{ExtensionStorageError, IdbExtensionLoader},
};

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_install_extension() {
        let foxe = fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap();
        let loader = IdbExtensionLoader::new("local");

        let mut db = IndexedDb::open("test", 1).await.unwrap();

        let ext_info: ExtensionInfo = serde_json::from_str(&foxe).unwrap();
        let foxe_data: Vec<u8> = fs::read(EXT_FILE_TURTLESIM).unwrap();
        let stored_extension = StoredExtension {
            info: ext_info,
            content: foxe_data,
        };

        db.put("metadata_store_name", &stored_extension.info).await.unwrap();
        db.put("extension_store_name", &stored_extension).await.unwrap();

        assert_eq!(
            loader.install_extension(&foxe_data).await.unwrap(),
            stored_extension
        );
    }

    #[tokio::test]
    async fn test_install_private_extension() {
        let foxe = fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap();
        let info: ExtensionInfo = serde_json::from_str(&foxe).unwrap();

        let zip = JsZip::new();
        zip.file("package.json", json!(info.package_json));
        zip.file("extension", foxe);
        let mock_foxe_data = zip.generate_async().await.unwrap();

        let loader = IdbExtensionLoader::new("org");

        let stored_extension = StoredExtension {
            info,
            content: mock_foxe_data,
        };

        db.put("metadata_store_name", &stored_extension.info).await.unwrap();
        db.put("extension_store_name", &stored_extension).await.unwrap();

        let retrieved_info = loader.get_extension(info.id.clone()).await.unwrap().info;

        assert_eq!(retrieved_info.namespace, "org");
        assert_eq!(retrieved_info.qualified_name, info.displayName);
    }

    #[tokio::test]
    async fn test_install_missing_package_json() {
        let zip = JsZip::new();
        zip.file("extension", fs::read("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap());
        let mock_foxe_data = zip.generate_async().await.unwrap();

        let loader = IdbExtensionLoader::new("local");

        assert!(loader.install_extension(&mock_foxe_data).await.is_err(), "Should throw error");
    }

    #[tokio::test]
    async fn test_install_missing_readme_or_changelog() {
        let foxe = fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap();
        let info: ExtensionInfo = serde_json::from_str(&foxe).unwrap();

        let zip = JsZip::new();
        zip.file("package.json", json!(info.package_json));
        zip.file("extension", foxe);
        zip.remove_file("README.md");
        zip.remove_file("CHANGELOG.md");
        let mock_foxe_data = zip.generate_async().await.unwrap();

        let loader = IdbExtensionLoader::new("local");

        assert!(loader.install_extension(&mock_foxe_data).await.is_err(), "Should throw error");
    }

    #[tokio::test]
    async fn test_load_extension() {
        let foxe = fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap();
        let extension: StoredExtension = serde_json::from_str(&foxe).unwrap();

        let mut db = IndexedDb::open("test", 1).await.unwrap();

        db.put("metadata_store_name", &extension.info).await.unwrap();
        db.put("extension_store_name", &extension).await.unwrap();

        assert_eq!(
            loader.load_extension(extension.info.id.clone()).await.unwrap(),
            extension.content
        );
    }

    #[tokio::test]
    async fn test_load_nonexistent_extension() {
        let loader = IdbExtensionLoader::new("local");

        assert!(loader.load_extension(BasicBuilder.string()).await.is_err(), "Should throw error");
    }

    #[tokio::test]
    async fn test_load_missing_content() {
        let extension: StoredExtension = serde_json::from_str(&fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap()).unwrap();

        let mut db = IndexedDb::open("test", 1).await.unwrap();
        db.put("metadata_store_name", &extension.info).await.unwrap();

        assert!(loader.load_extension(extension.info.id.clone()).await.is_err(), "Should throw error");
    }

    #[tokio::test]
    async fn test_load_missing_main_script() {
        let foxe = fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap();
        let info: ExtensionInfo = serde_json::from_str(&foxe).unwrap();

        let zip = JsZip::new();
        zip.file("package.json", json!(info.package_json));
        zip.file("extension", foxe);
        zip.remove_file("extension");
        let mock_foxe_data = zip.generate_async().await.unwrap();

        let loader = IdbExtensionLoader::new("local");

        assert!(loader.load_extension(&mock_foxe_data).await.is_err(), "Should throw error");
    }

    #[tokio::test]
    async fn test_get_extension() {
        let foxe = fs::read_to_string("../../test/fixtures/lichtblick.suite-extension-turtlesim-0.0.1.foxe").unwrap();
        let expected_info: ExtensionInfo = serde_json::from_str(&foxe).unwrap();

        let mut db = IndexedDb::open("test", 1).await.unwrap();

        db.put("metadata_store_name", &expected_info).await.unwrap();
        db.put("extension_store_name", &StoredExtension {
            info: expected_info,
            content: fs::read(EXT_FILE_TURTLESIM).unwrap(),
        }).await.unwrap();

        assert_eq!(
            loader.get_extension(expected_info.id.clone()).await.unwrap(),
            expected_info
        );
    }

    #[tokio::test]
    async fn test_uninstall_extension() {
        let extension_id = BasicBuilder.string();
        let mut db = IndexedDb::open("test", 1).await.unwrap();

        db.put("metadata_store_name", &StoredExtension {
            info: {
                id: extension_id.clone(),
            },
        }).await.unwrap();
        db.put("extension_store_name", &StoredExtension {
            info: {
                id: extension_id.clone(),
            },
            content: fs::read(EXT_FILE_TURTLESIM).unwrap(),
        }).await.unwrap();

        loader.uninstall_extension(extension_id).await.unwrap();

        assert_eq!(
            db.get("metadata_store_name", extension_id).await.unwrap(),
            None
        );
        assert_eq!(
            db.get("extension_store_name", extension_id).await.unwrap(),
            None
        );
    }
}
```