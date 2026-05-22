```rust
use std::sync::Arc;

pub struct VelodynePlayer {
    id: String,
    port: u16,
    listener: Option<Arc<dyn Fn(PlayerState) -> anyhow::Result<()>>>,
    socket: Option<std::net::UdpSocket>,
    seq: usize,
    total_bytes_received: usize,
    closed: bool,
    topic: Topic,
    topics: Vec<Topic>,
    topic_stats: std::collections::HashMap<String, TopicStats>,
    start: time::Instant,
    packets: Vec<RawPacket>,
    parsed_messages: Vec<MessageEvent>,
    metrics_collector: Arc<dyn PlayerMetricsCollectorInterface>,
    presence: PlayerPresence,
    alerts: Vec<PlayerAlert>,
}

impl VelodynePlayer {
    fn new(opts: VelodynePlayerOpts) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        log::info!("initializing VelodynePlayer on port {}", opts.port.unwrap_or(DEFAULT_VELODYNE_PORT));
        self.metrics_collector.clone();
        self.start = time::Instant::now();
        self.presence = PlayerPresence.INITIALIZING;
        Self {
            id,
            port: opts.port.unwrap_or(DEFAULT_VELODYNE_PORT),
            listener: None,
            socket: None,
            seq: 0,
            total_bytes_received: 0,
            closed: false,
            topic: Topic::new("velodyne_points", "velodyne_msgs/VelodyneScan"),
            topics: vec![self.topic.clone()],
            topic_stats: std::collections::HashMap::new(),
            start: time::Instant::now(),
            packets: Vec::new(),
            parsed_messages: Vec::new(),
            metrics_collector: Arc::new(opts.metrics_collector),
            presence: PlayerPresence.INITIALIZING,
            alerts: Vec::new(),
        }
    }

    async fn open(&mut self) {
        if self.closed {
            return;
        }
        self.presence = PlayerPresence.PRESENT;
        self.emit_state();

        if self.socket.is_none() {
            let net = std::net::UdpSocket::bind(("0.0.0.0", self.port)).await.unwrap();
            self.socket = Some(net);
            self.socket
                .unwrap()
                .set_nonblocking(true)
                .unwrap();
            self.socket
                .unwrap()
                .set_read_timeout(std::time::Duration::from_secs(1))
                .unwrap();

            self.socket
                .unwrap()
                .register_receiver(self.handle_message.bind())
                .await
                .unwrap();
        } else {
            if let Err(err) = self.socket.as_mut().unwrap().close() {
                log::error!("Failed to close socket: {}", err);
            }
        }

        match self.socket.as_mut().unwrap().bind(("0.0.0.0", self.port)) {
            Ok(_) => log::debug!("Bound Velodyne UDP listener socket to port {}", self.port),
            Err(err) => {
                self.add_alert(PROBLEM_SOCKET_ERROR, "Could not bind to the Velodyne UDP data port");
            }
        }
    }

    fn handle_message(&mut self, data: &[u8], remote_info: std::net::SocketAddr) {
        let receive_time = time::Instant::now();
        let date = date::Date::from_utc(receive_time.to_std().unwrap(), &time::Utc);
        let top_of_hour = date.with_hms(0, 0, 0);

        self.total_bytes_received += data.len() as usize;
        self.presence = PlayerPresence.PRESENT;

        if self.seq == 0 {
            // this.#metrics_collector.record_time_to_first_msgs();
        }

        let raw_packet = RawPacket::new(data.to_vec());

        let frequency = RPM / 60.0;
        let rate =
            raw_packet.return_mode() == ReturnMode::DualReturn
                ? packet_rate(raw_packet.infer_model().unwrap_or(Model::HDL64E)) * 2
                : packet_rate(raw_packet.infer_model().unwrap_or(Model::HDL64E));
        let num_packets = (rate / frequency).ceil();

        self.packets.push(raw_packet);
        if self.packets.len() >= num_packets {
            let message = MessageEvent {
                header: Header {
                    seq: self.seq += 1,
                    stamp: receive_time.to_std().unwrap(),
                    frame_id: remote_info.ip().to_string(),
                },
                packets: self.packets.iter().map(|raw| raw_packet_to_ros(raw, top_of_hour)).collect(),
            };

            let size_in_bytes = self
                .packets
                .iter()
                .fold(0, |acc, packet| acc + packet.data.len());
            let msg: MessageEvent = {
                topic: TOPIC_NAME.to_string(),
                receive_time,
                message,
                size_in_bytes,
                schema_name: TOPIC.schema_name().unwrap_or(""),
            };
            self.parsed_messages.push(msg);
            self.packets.clear();

            // Update the message count
            let stats = self.topic_stats.get(&TOPIC_NAME).unwrap_or(&mut TopicStats {
                num_messages: 0,
                first_message_time: None,
                last_message_time: None,
            });
            stats.num_messages += 1;
            stats.first_message_time = Some(receive_time);
            stats.last_message_time = Some(receive_time);

            self.emit_state();
        }
    }

    fn add_alert(&mut self, id: &str, alert: PlayerAlert) {
        self.alerts.push(alert);
        if let Some(ref mut listener) = self.listener {
            listener(self.clone());
        }
    }

    fn clear_alert(&mut self, id: &str) {
        if let Some(alert) = self.alerts_by_id.get_mut(id).unwrap() {
            alert.severity = "cleared";
        }
        if let Some(ref mut listener) = self.listener {
            listener(self.clone());
        }
    }

    fn emit_state(&mut self) {
        if let Some(ref mut listener) = self.listener && !self.closed {
            listener(self.clone()).unwrap();
        }
        if let Some(timer) = &mut self.emit_timer {
            *timer = std::time::Instant::now().checked_add(std::time::Duration::from_secs(1)).unwrap();
        } else {
            self.emit_timer = Some(std::time::Instant::now().checked_add(std::time::Duration::from_secs(1)).unwrap());
            let _ = self.socket.as_mut().unwrap().set_read_timeout(std::time::Duration::from_secs(1));
            let _ = self.socket.as_mut().unwrap().register_receiver(self.handle_message.bind());
        }
    }

    fn set_listener(&mut self, listener: Arc<dyn Fn(PlayerState) -> anyhow::Result<()>>) {
        self.listener = Some(listener);
    }

    fn close(&mut self) {
        self.closed = true;
        if let Some(ref mut socket) = self.socket {
            socket.shutdown(std::net::Shutdown::Both).unwrap();
        }
        if let Some(ref mut listener) = &mut self.listener {
            listener(self.clone());
        }
        // self.metrics_collector.close();
        self.total_bytes_received = 0;
        self.seq = 0;
        self.packets.clear();
        self.parsed_messages.clear();
    }

    fn set_subscriptions(_subscriptions: Vec<SubscribePayload>) {}

    fn set_publishers(_publishers: Vec<AdvertiseOptions>) {
        // no-op
    }

    fn set_parameter(_key: &str, _value: ParameterValue) -> anyhow::Result<()> {
        Err(anyhow::Error::new("Parameter modification is not supported for VelodynePlayer"))
    }

    fn publish(_request: PublishPayload) -> anyhow::Result<()> {
        Err(anyhow::Error::new("Publishing is not supported for VelodynePlayer"))
    }

    async fn call_service() -> anyhow::Result<unknown> {
        Err(anyhow::Error::new("Service calls are not supported for VelodynePlayer"))
    }

    fn set_global_variables(_global_variables: GlobalVariables) {}

    fn get_batch_iterator() -> Option<Box<dyn BatchIterator>> {
        None
    }
}
```