```rust
use std::sync::{Arc, Mutex};
use futures::{StreamExt, AsyncReadExt};
use reqwest::{Client, RequestBuilder};

const LOG = log::Logger::new("fetch_reader");

pub struct FetchReader {
    client: Arc<Client>,
    url: String,
    reader: Option<tokio_util::io::StreamReader<Vec<u8>, ()>>,
}

impl FetchReader {
    pub fn new(url: &str) -> Self {
        let client = Arc::new(Client::new());
        Self {
            client,
            url: url.to_string(),
            reader: None,
        }
    }

    async fn get_reader(&self) -> Option<tokio_util::io::StreamReader<Vec<u8>, ()>> {
        if let Some(reader) = self.reader.clone() {
            return reader;
        }

        let response = self.client.get(&self.url).await?;
        if !response.status().is_success() {
            let status_text = response.text().await.unwrap_or("N/A");
            log.error!("GET <{}> failed with status {}", &self.url, response.status(), &status_text);
            return None;
        }

        let reader = response.bytes_stream().map_ok(|chunk| chunk.to_vec()).boxed();
        self.reader = Some(reader);
        reader
    }

    pub async fn read(&mut self) {
        if let Some(mut reader) = self.get_reader().await {
            while !reader.is_done() {
                let chunk = reader.next().await.expect("Failed to read chunk");
                log.info!("Received chunk: {:?}", chunk);
                // Process the chunk here
            }
            log.info!("Stream is finished");
            self.reader.take();
        } else {
            log.error!("Failed to get reader");
        }
    }

    pub fn destroy(&mut self) {
        if let Some(mut reader) = self.get_reader().await {
            reader.cancel();
            self.reader.take();
        }
    }
}
```