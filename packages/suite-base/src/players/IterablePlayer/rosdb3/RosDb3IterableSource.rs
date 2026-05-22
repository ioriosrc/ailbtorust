```rust
use std::fs;
use std::io::BufReader;
use std::path::Path;

use crate::{
    database::{Database, DatabaseError},
    messages::{Message, Timestamp},
};
use crate::{
    protocol::{decode_message, decode_topic_statistics, decode_type_definition},
    types::RosDatatype,
};

pub struct RosDb3IterableSource {
    files: Vec<PathBuf>,
}

impl RosDb3IterableSource {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self { files }
    }

    async fn initialize(&mut self) -> Result<(), DatabaseError> {
        // Load the SQL.js library and initialize it.
        let sql_wasm = fs::read("path/to/sql-js/dist/sql-wasm.wasm")?;
        crate::database::initialize_with_wasm(sql_wasm)?;

        // Open each file in the bag and read its contents.
        for path in &self.files {
            let db = Database::open(path)?;
            self.bag = Some(db);
            break; // We only need to open the first database
        }

        if self.bag.is_none() {
            return Err(DatabaseError::NoDatabaseFound);
        }

        // Read the time range and topic definitions from the bag.
        let start_time = self.bag.as_ref().unwrap().time_range()?;
        let end_time = start_time.add(std::chrono::Duration::seconds(1));
        let topics = self.bag.as_ref().unwrap().read_topics()?;

        // Decode the type definitions.
        let mut datatypes: std::collections::HashMap<String, RosDatatype> = std::collections::HashMap::new();
        for topic_def in &topics {
            if let Some(def) = decode_type_definition(topic_def.type_name())? {
                datatypes.insert(topic_def.name(), def);
            } else {
                return Err(DatabaseError::UnsupportedDatatype);
            }
        }

        // Initialize the topic stats and alerts.
        let mut topic_stats = std::collections::HashMap::new();
        for (_, topic) in topics.iter() {
            topic_stats.insert(topic.name().clone(), TopicStats { num_messages: 0 });
        }

        let start = Timestamp::from(start_time);
        let end = Timestamp::from(end_time);

        // Read the messages from the bag.
        let mut msg_iterator = self.bag.as_ref().unwrap().read_messages(&start, &end)?;

        while let Some(msg) = msg_iterator.next()? {
            yield MessageEvent::new(
                "message-event",
                msg.timestamp,
                decode_message(&msg.data, datatypes.get(&msg.topic_type).ok_or(DatabaseError::UnsupportedDatatype)?),
                msg.data.len() as u64,
                msg.topic_name().to_string(),
            );
        }

        Ok(())
    }
}
```