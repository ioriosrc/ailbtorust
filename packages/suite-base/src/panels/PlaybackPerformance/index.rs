```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(message: &str);
}

#[wasm_bindgen]
pub struct PlaybackPerformance {
    timestamp: i64,
    active_data: Option<PlayerStateActiveData>,
    perf_points: HashMap<String, Vec<SparklinePoint>>,
}

impl PlaybackPerformance {
    pub fn new(timestamp: i64, active_data: Option<PlayerStateActiveData>) -> Self {
        PlaybackPerformance {
            timestamp,
            active_data,
            perf_points: HashMap::new(),
        }
    }

    pub fn update(&mut self, player_state: PlayerStateActiveData) {
        if let Some(active_data) = &self.active_data {
            if player_state.is_playing() && player_state.last_seek_time == active_data.last_seek_time {
                let elapsed_player_time =
                    (player_state.current_time - active_data.current_time) * 1000;
                self.perf_points.entry("speed".to_string()).or_insert_with(Vec::new).push({
                    value: elapsed_player_time / player_state.render_time(),
                    timestamp: self.timestamp,
                });
                self.perf_points.entry("framerate".to_string()).or_insert_with(Vec::new).push({
                    value: 1000.0 / player_state.render_time(),
                    timestamp: self.timestamp,
                });
            }
            let new_bytes_received = player_state.total_bytes_received - self.active_data.unwrap().total_bytes_received;
            let new_megabits_received = (8 * new_bytes_received) / 1e6;
            let megabits_per_second = new_megabits_received / (player_state.render_time() as f32);
            self.perf_points.entry("megabits_per_second".to_string()).or_insert_with(Vec::new).push({
                value: megabits_per_second,
                timestamp: self.timestamp,
            });
        }
    }

    pub fn render(&self) -> String {
        let mut html = "<div>";
        for (key, points) in &self.perf_points {
            html.push_str(&format!(
                "<div><h2>{}</h2><Sparkline points={:?} maximum={} decimalPlaces={}</div>",
                key,
                points,
                1.6,
                2
            ));
        }
        html.push_str("</div>");
        html
    }
}

#[wasm_bindgen]
pub fn main() {
    alert("Hello, world!");
}
```