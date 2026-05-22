```rust
use async_db::DB;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ExtensionInfo {
    // Define the fields of ExtensionInfo here
}

#[derive(Serialize, Deserialize)]
struct StoredExtension {
    info: ExtensionInfo,
    // Define the fields of StoredExtension here
}

struct IDBExtensionStorage {
    namespace: String,
    db: DB<ExtensionsDB>,
}

impl IDBExtensionStorage {
    pub fn new(namespace: String) -> Self {
        let connection = async_db::open(&[format!("{}-{}", DATABASE_BASE_NAME, &namespace)].join("-"), 1)
            .await
            .unwrap();
        IDBExtensionStorage { namespace, db }
    }

    async fn list() -> Vec<ExtensionInfo> {
        let start = std::time::Instant::now();
        let records: Vec<_> = self.db.get_all(METADATA_STORE_NAME).await.unwrap();
        println!(
            "Loaded {} extensions in {:.2} ms",
            records.len(),
            (std::time::Instant::now() - start).as_secs_f64()
        );
        records
    }

    async fn get(id: &str) -> Option<StoredExtension> {
        let start = std::time::Instant::now();
        let extension = self.db.get(EXTENSION_STORE_NAME, id).await.unwrap();
        println!("Getting extension {} took {:.2} ms", id, (std::time::Instant::now() - start).as_secs_f64());
        Some(extension)
    }

    async fn put(&self, extension: StoredExtension) -> StoredExtension {
        let start = std::time::Instant::now();
        self.db
            .put(METADATA_STORE_NAME, extension.info)
            .await
            .unwrap();
        self.db.put(EXTENSION_STORE_NAME, extension).await.unwrap();
        println!(
            "Stored extension {} in {:.2} ms",
            &extension.info.id,
            (std::time::Instant::now() - start).as_secs_f64()
        );
        extension
    }

    async fn delete(&self, id: &str) {
        let start = std::time::Instant::now();
        self.db.delete(METADATA_STORE_NAME, id).await.unwrap();
        self.db.delete(EXTENSION_STORE_NAME, id).await.unwrap();
        println!("Deleted extension {} in {:.2} ms", id, (std::time::Instant::now() - start).as_secs_f64());
    }
}

struct ExtensionsDB {
    metadata: Vec<(String, ExtensionInfo)>,
    extensions: Vec<(String, StoredExtension)>,
}
```