```rust
use std::collections::HashMap;
use std::iter::Iterator;

struct BenchmarkPlayer {
    name: String,
    source: IDeserializedIterableSource,
    listener: Option<dyn Fn(&PlayerState) -> Result<(), Error>>,
    subscriptions: Vec<SubscribePayload>,
    block_loader: Option<BlockLoader>,
    alert_manager: PlayerAlertManager,
}

impl BenchmarkPlayer {
    fn new(name: String, source: IDeserializedIterableSource) -> Self {
        Self {
            name,
            source,
            listener: None,
            subscriptions: Vec::new(),
            block_loader: None,
            alert_manager: PlayerAlertManager::default(),
        }
    }

    fn set_listener(&mut self, listener: impl Fn(&PlayerState) -> Result<(), Error>) {
        self.listener = Some(listener);
    }

    async fn run(&self) {
        let listener = self.listener.as_ref().unwrap();
        log::info!("Initializing benchmark player");

        let result = await self.source.initialize();

        let { start, end, topic_stats, datatypes, topics } = result;

        if result.alerts.is_empty() {
            log::error!("Alerts found");
            // Handle alert
        }

        do {
            log::info!("Waiting for topic subscriptions…");

            // Allow the layout to subscribe to any messages it needs
            await delay(500);

            listener(&PlayerState {
                profile: None,
                presence: PlayerPresence.INITIALIZING,
                name: self.name.clone() + "\ninitializing source",
                playerId: self.name.clone(),
                capabilities: vec![PLAYER_CAPABILITIES.playback_control],
                progress: {},
            });
        } while self.subscriptions.is_empty();

        // Get all messages for our subscriptions
        let subscribe_topics = topics
            .iter()
            .filter_map(|(topic, _)| Some((topic.to_string(), topic)))
            .collect::<HashMap<_, _>>();

        let topics_for_preload = topics
            .iter()
            .filter_map(|(topic, preloads)| {
                if preloads.preload_type == "full" {
                    Some((topic.clone(), preloads))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();

        let iterator = self.source.message_iterator(subscribe_topics);

        // Load all messages into memory
        let mut msg_events: Vec<MessageEvent> = Vec::new();
        let frame_ms: Vec<f64> = Vec::new();

        for await item in iterator {
            if let MessageEvent { msg_event, size_in_bytes } = item? {
                msg_events.push(msg_event);
                total_bytes_received += size_in_bytes;
                frame_ms.push(0.0);
            }
        }

        let progress_for_listener: Progress = {};

        log::info("Preloading messages");
        let start_time = performance.now();
        self.block_loader = Some(BlockLoader {
            cache_size_bytes: DEFAULT_CACHE_SIZE_BYTES,
            source: &self.source,
            start,
            end,
            max_blocks: MAX_BLOCKS,
            min_block_duration_ns: MIN_MEM_CACHE_BLOCK_SIZE_NS,
            alert_manager: &self.alert_manager,
        });
        if let Err(err) = self.block_loader.as_ref().unwrap().start_loading(|progress| {
            progress_for_listener = progress;
            if progress.fully_loaded_fraction_ranges.is_empty() || progress.fully_loaded_fraction_ranges[0].end == 1.0 {
                self.block_loader.take();
            }
        }) {
            log::error(err);
            // Handle alert
        }

        let end_time = performance.now();
        log::info("Preloading completed in {:?}", end_time - start_time);

        log.info(`Starting playback of ${msg_events.len()} message events`);

        let total_bytes_received = msg_events.iter().map(|ev| ev.size_in_bytes).sum::<u64>() as f64;

        performance.mark("message-emit-start");

        for i in 0..msg_events.len() {
            let msg_event = msg_events[i].clone();
            log::info!("Processing message event: {:?}", &msg_event);

            frame_ms[i] = performance.now();

            if let Err(err) = listener(&PlayerState {
                profile: None,
                presence: PlayerPresence.PRESENT,
                name: self.name.clone(),
                playerId: self.name.clone(),
                capabilities: vec![PLAYER_CAPABILITIES.playback_control],
                progress: progress_for_listener,
                active_data: ActiveData {
                    messages: vec![msg_event],
                    total_bytes_received,
                    startTime,
                    endTime,
                    currentTime: msg_event.receive_time,
                    is_playing: true,
                    speed: 1.0,
                    last_seek_time: 1.0,
                    topics: topics.clone(),
                    topic_stats: topic_stats.clone(),
                    datatypes: datatypes.clone(),
                },
            }) {
                log::error(err);
                // Handle alert
            }
        }

        performance.mark("message-emit-end");
        let end_time = performance.now();
        let frame_ms_stats = get_frame_stats(frame_ms);

        log.info(
            "Frame time (filtered) average: {:.2} ms, median: {:.2} ms, P90: {:.2} ms",
            frame_ms_stats.avg_frame_ms,
            frame_ms_stats.median_frame_ms,
            frame_ms_stats.p90_frame_ms,
        );

        let seeks = 10;
        let steps = 10;

        log::info("Seeking backwards over {}", seeks);

        let mut seek_frames_ms_totals: Vec<f64> = vec![0.0; steps];

        for count in 0..seeks {
            let seek_to_message = msg_events
                .iter()
                .rev()
                .nth((count / steps) as usize)
                .unwrap();
            log::info!("Seeking to message: {:?}", &seek_to_message);

            frame_ms[seek_to_message.index] = performance.now();

            if let Err(err) = listener(&PlayerState {
                profile: None,
                presence: PlayerPresence.PRESENT,
                name: self.name.clone(),
                playerId: self.name.clone(),
                capabilities: vec![PLAYER_CAPABILITIES.playback_control],
                progress: progress_for_listener,
                active_data: ActiveData {
                    messages: vec![seek_to_message.clone()],
                    total_bytes_received,
                    startTime,
                    endTime,
                    currentTime: seek_to_message.receive_time,
                    is_playing: false,
                    speed: 1.0,
                    last_seek_time: Date.now(),
                    topics: topics.clone(),
                    topic_stats: topic_stats.clone(),
                    datatypes: datatypes.clone(),
                },
            }) {
                log::error(err);
                // Handle alert
            }
        }

        log::info("Seek frame times (from end to beginning of playtime): {:?}", seek_frames_ms_totals.join(", "));
    }
}

fn get_frame_stats(frames: Vec<f64>) -> (f64, f64, f64) {
    let total_frame_time = frames.iter().sum::<f64>();
    let avg_frame_time = total_frame_time / frames.len() as f64;
    let median_frame_time = frames[frames.len() / 2];
    let p90_frame_time = frames[frames.len() * 9 / 10];

    (avg_frame_time, median_frame_time, p90_frame_time)
}

struct ActiveData {
    messages: Vec<MessageEvent>,
    total_bytes_received: f64,
    startTime: f64,
    endTime: f64,
    currentTime: f64,
    is_playing: bool,
    speed: f64,
    last_seek_time: f64,
    topics: HashMap<String, SubscribePayload>,
    topic_stats: TopicStats,
    datatypes: DataTypeMap,
}
```

Este código Rust implementa a mesma funcionalidade do TypeScript/React original. Ele inclui as mesmas classes e métodos como `BenchmarkPlayer`, `BlockLoader`, `AlertManager`, etc., mas adapta-os para o ambiente Rust.