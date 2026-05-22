```rust
use std::fs::{self, create_dir_all};
use std::path::Path;

const LOG = "local_file_storage";

pub struct LocalFileStorage {
    datastores_dir_path: PathBuf,
}

impl LocalFileStorage {
    pub fn new() -> Self {
        let app_data_dir = dirs::appdata_local_dir().expect("Failed to get user data path");
        let datastore_dir_name = DATASTORES_DIR_NAME;
        let datastores_dir_path = app_data_dir.join(datastore_dir_name);

        create_dir_all(&datastores_dir_path).expect("Failed to create datastore directory");

        LocalFileStorage { datastores_dir_path }
    }

    async fn list(&self, datastore: &str) -> Vec<String> {
        if !self.datastores_dir_path.exists() {
            return vec![];
        }

        let datastore_dir = self.datastores_dir_path.join(datastore);
        if !datastore_dir.exists() {
            return vec![];
        }

        fs::read_dir(&datastore_dir)
            .await
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok())
            .map(|e| e.path().file_name().unwrap_or_default().to_string_lossy().into_owned())
            .collect()
    }

    async fn all(&self, datastore: &str) -> Vec<Vec<u8>> {
        let datastore_dir = self.datastores_dir_path.join(datastore);
        if !datastore_dir.exists() {
            return vec![];
        }

        fs::read_all(&datastore_dir).await.expect("Failed to read file")
    }

    async fn get(
        &self,
        datastore: &str,
        key: &str,
        encoding: Option<&str>,
    ) -> Result<Vec<u8>, std::io::Error> {
        let filepath = self.#make_filepath(datastore, key);
        if !filepath.exists() {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }

        match encoding {
            Some("utf-8") => fs::read_to_string(filepath).await.map(|s| s.into_bytes()),
            None => Ok(fs::read_all(filepath).await?),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unsupported encoding",
            )),
        }
    }

    async fn put(&self, datastore: &str, key: &str, value: &[u8]) -> Result<(), std::io::Error> {
        let filepath = self.#make_filepath(datastore, key);
        fs::write(filepath, value).await?;
        Ok(())
    }

    async fn delete(&self, datastore: &str, key: &str) -> Result<(), std::io::Error> {
        let filepath = self.#make_filepath(datastore, key);
        if !filepath.exists() {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }
        fs::remove_file(filepath).await?;
        Ok(())
    }

    async fn #make_filepath(&self, datastore: &str, key: &str) -> PathBuf {
        let datastore_dir = self.datastores_dir_path.join(datastore);
        create_dir_all(&datastore_dir).expect("Failed to create datastore directory");

        let sanitized_key = sanitize_key(key);
        if sanitized_key.is_empty() {
            return datastore_dir.join(".");
        }

        datastore_dir.join(sanitized_key)
    }
}

fn sanitize_key(key: &str) -> String {
    key.chars()
        .filter(|c| c.is_alphanumeric() || c == '-' || c == '_')
        .collect::<String>()
}
```