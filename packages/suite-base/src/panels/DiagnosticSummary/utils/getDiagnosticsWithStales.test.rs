```rust
use chrono::NaiveTime;
use std::{collections::HashMap, time::Duration};

struct Time {
    sec: i64,
    nsec: u32,
}

fn get_diagnostics_with_stales(
    diagnostics_by_hardware_id: &mut HashMap<String, HashMap<String, DiagnosticInfo>>,
    stale_time: Duration,
) -> HashMap<String, HashMap<String, DiagnosticInfo>> {
    let now = NaiveTime::now().into();
    let stale_time = Utc.timestamp(now.timestamp() - stale_time.as_secs(), now.nanosecond());

    diagnostics_by_hardware_id.iter_mut().for_each(|(hardware_id, hardware_diagnostics)| {
        hardware_diagnostics.iter_mut().for_each(|(diagnostic_name, diagnostic_info)| {
            let stamp: NaiveTime = NaiveTime::from_timestamp(diagnostic_info.stamp.sec(), diagnostic_info.stamp.nsec());

            if stamp < stale_time {
                diagnostic_info.status.level = LEVELS.STALE;
            }
        });
    });

    diagnostics_by_hardware_id.clone()
}

fn create_diagnostic_info(stamp: NaiveTime, level: i32) -> DiagnosticInfo {
    DiagnosticInfo {
        stamp,
        status: DiagnosticInfo::new_status_message(level),
    }
}

struct DiagnosticInfo {
    stamp: NaiveTime,
    status: DiagnosticInfoStatus,
}

enum DiagnosticInfoStatus {
    OK,
    STALE,
    // Add other levels if needed
}

#[derive(Debug, Clone)]
struct BasicBuilder;

#[derive(Debug, Clone)]
struct RosTimeBuilder {
    time: Duration,
}

impl RosTimeBuilder {
    fn time(&self) -> NaiveTime {
        let secs = self.time.as_secs();
        let nsecs = self.time.subsec_nanos() as u32;
        NaiveTime::from_hms(secs, 0, 0, nsecs)
    }
}

impl BasicBuilder {
    fn string(&self) -> String {
        format!("diagnostic_{}", rand::thread_rng().gen_range(1..10))
    }
}

struct DiagnosticsBuilder;

impl DiagnosticsBuilder {
    fn info(&self) -> DiagnosticInfo {
        DiagnosticInfo::new_status_message(1)
    }

    fn new_status_message(level: i32) -> DiagnosticInfoStatus {
        match level {
            1 => DiagnosticInfoStatus::OK,
            _ => DiagnosticInfoStatus::STALE, // Add other levels if needed
        }
    }
}

fn main() {
    let stale_time = Duration::from_secs(50);

    let mut diagnostics_by_hardware_id: HashMap<String, HashMap<String, DiagnosticInfo>> = HashMap::new();

    let time1 = RosTimeBuilder::time();
    let time2 = RosTimeBuilder::time();

    let diagnostic_info1 = create_diagnostic_info(time1.into(), 1); // become stale
    let diagnostic_info2 = create_diagnostic_info(time2.into(), 4); // keep level

    diagnostics_by_hardware_id.insert("hardware_1".to_string(), HashMap::new());
    diagnostics_by_hardware_id.get_mut("hardware_1").unwrap().insert(
        "diagnostic_1".to_string(),
        diagnostic_info1,
    );

    let hardware_id2 = BasicBuilder.string();
    let diagnostic_info3 = create_diagnostic_info(time2.into(), 4); // keep level

    diagnostics_by_hardware_id.insert(hardware_id2.to_string(), HashMap::new());
    diagnostics_by_hardware_id.get_mut(&hardware_id2).unwrap().insert(
        "diagnostic_3".to_string(),
        diagnostic_info3,
    );

    let result = get_diagnostics_with_stales(&mut diagnostics_by_hardware_id, stale_time);

    println!("{:?}", result);
}
```