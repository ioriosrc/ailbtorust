```rust
use std::sync::{Arc, RwLock};
use std::thread;

struct MockRosClient {
    topics: Vec<String>,
    types: Vec<String>,
    typedefs_full_text: Vec<String>,
    connect_callback: Option<Box<dyn FnOnce()>>,
    messages: Vec<serde_json::Value>,
}

impl MockRosClient {
    pub fn setup(&mut self, config: SetupConfig) {
        self.topics = config.topics;
        self.types = config.types;
        self.typedefs_full_text = config.typedefs;
        self.connect_callback = Some(Box::new(config.connect_callback));
        self.messages = config.messages;

        if let Some(callback) = &self.connect_callback {
            callback();
        }
    }

    pub fn on(&mut self, op: String, callback: Box<dyn FnOnce()> {
        if op == "connection" {
            self.connect_callback = Some(callback);
        }
    })

    pub fn close(&mut self) {}

    pub fn get_topics_and_raw_types(&self, callback: impl FnOnce(&GetTopicsAndRawTypesResponse)) {
        callback(GetTopicsAndRawTypesResponse {
            topics: &self.topics,
            types: &self.types,
            typedefs_full_text: &self.typedefs_full_text,
        });
    }

    pub fn get_messages_by_topic_name(&self, topic_name: String) -> Vec<Message> {
        self.messages
            .iter()
            .filter(|message| message["topic"].as_str().unwrap() == topic_name)
            .map(|msg| Message::from(msg.clone()))
            .collect()
    }

    pub fn get_nodes(&self, callback: impl FnOnce(&GetNodesResponse)) {
        callback(GetNodesResponse { nodes: &Vec::new() });
    }

    pub fn get_node_details(
        &self,
        _node: String,
        callback: impl FnOnce(&GetNodeDetailsResponse),
        _err_cb: impl FnOnce(&Error),
    ) {
        callback(GetNodeDetailsResponse { subscriptions: &Vec::new(), publications: &Vec::new(), services: &Vec::new() });
    }
}

struct SetupConfig {
    topics: Vec<String>,
    types: Vec<String>,
    typedefs: Vec<String>,
    connect_callback: Box<dyn FnOnce()>,
    messages: Vec<serde_json::Value>,
}

struct GetTopicsAndRawTypesResponse {
    topics: &[String],
    types: &[String],
    typedefs_full_text: &[String],
}

struct Message {
    topic: String,
    receive_time: Time,
    message: serde_json::Value,
}

struct GetNodesResponse {
    nodes: &[String],
}

struct GetNodeDetailsResponse {
    subscriptions: &[String],
    publications: &[String],
    services: &[String],
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker_instance = Arc::new(RwLock::new(MockRosClient {
        topics: vec!["/topic/A"],
        types: vec!["std_msgs/Header", "rosgraph_msgs/Log"],
        typedefs_full_text: vec![
            r#"std_msgs/Header header

            ================================================================================
            MSG: std_msgs/Header
            uint32 seq
            time stamp
            string frame_id"#,
        ],
    }));

    let mut player = RosbridgePlayer {
        url: "ws://some-url",
        metrics_collector: NoopMetricsCollector::new(),
        source_id: "rosbridge-websocket",
    };

    thread::spawn(move || {
        worker_instance.write().unwrap().setup({
            topics: vec!["/topic/A", "/topic/B"],
            types: vec!["std_msgs/Header", "text", "rosgraph_msgs/Log"],
            typedefs: vec![
                r#"std_msgs/Header header

                  ================================================================================
                  MSG: std_msgs/Header
                  uint32 seq
                  time stamp
                  string frame_id"#,
            ],
            messages: vec![
                serde_json::json!({
                    "topic": "/topic/A",
                    "receiveTime": { "sec": 100, "nsec": 0 },
                    "message": header_message({
                        "seq": 7643,
                        "stamp": { "sec": 1234, "nsec": 5678 },
                        "frame_id": "someFrameId",
                    }),
                }),
                serde_json::json!({
                    "topic": "/topic/B",
                    "receiveTime": { "sec": 100, "nsec": 0 },
                    "message": text_message({ text: "some text" }),
                }),
            ],
        });
    });

    player.set_subscriptions([{ topic: "/topic/A" }]);
    let mut sig = Arc::new(RwLock::new(()));

    player.set_listener(move |active_data| {
        let active_data = match active_data {
            Some(data) => data,
            None => return,
        };

        if let Some(topics) = &active_data.topics {
            assert_eq!(topics, vec!["/topic/A"]);
        }

        sig.write().unwrap().notify();
    });

    sig.read().unwrap().wait();

    player.set_subscriptions([{ topic: "/topic/B" }]);
    sig = Arc::new(RwLock::new(()));

    player.set_listener(move |active_data| {
        let active_data = match active_data {
            Some(data) => data,
            None => return,
        };

        if let Some(messages) = &active_data.messages {
            assert_eq!(messages.len(), 1);
            assert_eq!(
                messages[0],
                serde_json::json!({
                    "topic": "/topic/B",
                    "receiveTime": { "sec": 100, "nsec": 0 },
                    "message": text_message({ text: "some text" }),
                })
            );
        }

        sig.write().unwrap().notify();
    });

    sig.read().unwrap().wait();

    Ok(())
}
```