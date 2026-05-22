```rust
use log::error;
use rosbridge_rs::rostime::{self, Time};
use rosbridge_rs::service::ServiceRequest;

struct SinewavePlayer {
    name: String,
    start_time: Time,
    listener: Option<Box<dyn Fn(PlayerState) -> Box<dyn Future<Output = ()>>>>,
    datatypes: HashMap<String, RosDatatypes>,
}

impl SinewavePlayer {
    pub fn new() -> Self {
        SinewavePlayer {
            name: "sinewave".to_string(),
            start_time: rostime::from_date(chrono::Utc::now()),
            listener: None,
            datatypes: HashMap::new(),
        }
    }

    pub fn set_listener(&mut self, listener: Box<dyn Fn(PlayerState) -> Box<dyn Future<Output = ()>>>> {
        self.listener = Some(listener);
    }

    pub fn close(&self) {}

    pub async fn run(&mut self) {
        let listener = self.listener.take();
        if listener.is_none() {
            error!("Invariant: listener is not set");
            return;
        }

        log::info!("Initializing sinewave player");

        let state = PlayerState {
            profile: None,
            presence: PlayerPresence::PRESENT,
            name: self.name.clone(),
            playerId: self.name.clone(),
            capabilities: vec![],
            progress: {},
            url_state: {
                source_id: "sinewave",
            },
        };

        listener.unwrap()(state).await;

        let sinewave_count = 100;

        let topics = Vec::new();

        let start_time = rostime::from_date(chrono::Utc::now());

        for i in 0..sinewave_count {
            let topic_name = format!("sinewave_{i}");
            topics.push(Topic {
                name: topic_name,
                schema_name: "Sinewave".to_string(),
            });
        }

        let mut message_count = 0;
        loop {
            message_count += 1;

            let now = rostime::from_date(chrono::Utc::now());
            let value = f64::sin(now.to_sec());

            let messages: Vec<MessageEvent> = (0..sinewave_count)
                .map(|i| {
                    let topic_name = format!("sinewave_{i}");
                    MessageEvent {
                        receive_time: now,
                        topic: topic_name.clone(),
                        schema_name: "Sinewave".to_string(),
                        message: Some(serde_json::json!({ value: value + i * 0.1 })),
                        size_in_bytes: 0,
                    }
                })
                .collect();

            let frame_start_ms = std::time::Instant::now().as_secs_f64() * 1000.0;

            let state = PlayerState {
                profile: None,
                presence: PlayerPresence::PRESENT,
                name: self.name.clone(),
                playerId: self.name.clone(),
                capabilities: vec![],
                progress: {},
                active_data: ActiveData {
                    messages,
                    total_bytes_received: 0,
                    currentTime: now,
                    startTime: self.start_time,
                    is_playing: true,
                    speed: 1.0,
                    last_seek_time: 1,
                    endTime: now,
                    topics,
                    topic_stats: HashMap::new(),
                    datatypes: self.datatypes.clone(),
                },
            };

            listener.unwrap()(state).await;

            let frame_end_ms = std::time::Instant::now().as_secs_f64() * 1000.0;
            let frame_time_ms = frame_end_ms - frame_start_ms;

            BenchmarkStats::instance().record_frame_time(frame_time_ms);
        }
    }
}
```