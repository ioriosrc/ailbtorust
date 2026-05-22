```rust
use std::cmp::Ordering;

struct Sample {
    stamp: f64, // in seconds since Epoch
    value: f64,
}

struct BenchmarkStats {
    frame_times_ms: Vec<Sample>,
}

impl Default for BenchmarkStats {
    fn default() -> Self {
        Self { frame_times_ms: Vec::new() }
    }
}

impl BenchmarkStats {
    pub fn record_frame_time(&mut self, duration_ms: f64) {
        let now = std::time::Instant::now().elapsed();
        self.frame_times_ms.push(Sample {
            stamp: now.as_secs_f64(),
            value: duration_ms,
        });

        if self.frame_times_ms.len() >= 100 {
            let mut values = self.frame_times_ms.iter().map(|s| s.value).collect::<Vec<f64>>();
            values.sort_unstable();

            let total_frame_ms: f64 = values.iter().sum();
            let avg_frame_ms = total_frame_ms / values.len() as f64;

            let median_index = (values.len() - 1) / 2;
            let median_frame_ms = values.get(median_index).copied().unwrap_or(values[0]);

            let p90_index = (values.len() * 0.9) as usize;
            let p90_frame_ms = values.get(p90_index).copied().unwrap_or(values[0]);

            let stddev = std::f64::sqrt(values.iter().map(|&v| v - avg_frame_ms).map(|v| v.powi(2)).sum::<f64>() / values.len() as f64);

            log.info(&format!(
                "Frame time (filtered) average: {:.3}, median: {:.3}, P90: {:.3}, stddev: {:.3}",
                avg_frame_ms, median_frame_ms, p90_frame_ms, stddev
            ));

            let record = window.get_record_frame_times();
            if let Some(record_fn) = record {
                record_fn(self.frame_times_ms.clone());
            }

            self.frame_times_ms.clear();
        }
    }

    pub fn instance() -> Self {
        BenchmarkStats::default()
    }
}
```