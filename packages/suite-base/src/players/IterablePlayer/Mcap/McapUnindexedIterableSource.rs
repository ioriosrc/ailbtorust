```rust
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

struct McapStreamReader {
    // Implementation of McapStreamReader
}

struct McapTypes {
    Schema: Option<HashMap<String, String>>,
}

struct MessageEvent<T> {
    connectionId: i32,
    msgEvent: T,
}

struct Metadata {
    name: String,
    metadata: HashMap<String, String>,
}

struct Channel {
    topic: String,
    messageEncoding: String,
    schemaId: i32,
    metadata: HashMap<String, String>,
}

struct TopicWithDecodingInfo {
    name: String,
    messageEncoding: String,
    schemaName: Option<String>,
    schemaData: Option<Vec<u8>>,
    schemaEncoding: Option<String>,
}

struct RosDatatypes;

struct PlayerAlert;
struct TopicStats;

struct Initialization;

struct GetBackfillMessagesArgs;

struct ISerializedIterableSource {
    start: Time,
    end: Time,
    topics: Vec<TopicWithDecodingInfo>,
    datatypes: RosDatatypes,
    profile: Option<String>,
    alerts: Vec<PlayerAlert>,
    publishersByTopic: HashMap<String, HashSet<String>>,
    topicStats: HashMap<String, TopicStats>,
    metadata: Vec<Metadata>,
}

struct McapUnindexedIterableSource {
    options: Options,
}

impl McapUnindexedIterableSource {
    fn new(options: Options) -> Self {
        Self { options }
    }

    async fn initialize(&mut self) -> Initialization {
        // Implementation of initialization
    }

    async fn message_iterator(&self, args: MessageIteratorArgs) -> impl Iterator<Item = MessageEvent<Uint8Array>> + 'static {
        // Implementation of message_iterator
    }

    async fn get_backfill_messages(&self, args: GetBackfillMessagesArgs) -> Vec<MessageEvent<Uint8Array>> {
        // Implementation of get_backfill_messages
    }

    fn start(&self) -> Option<Time> {
        self.options.start
    }

    fn end(&self) -> Option<Time> {
        self.options.end
    }
}

struct Time {
    sec: i32,
    nsec: i32,
}

fn fromNanoSec(nanoseconds: u64) -> Time {
    let secs = nanoseconds / 1_000_000_000;
    let nsecs = nanoseconds % 1_000_000_000 as i32;
    Time { sec, nsec }
}

fn subtract(a: Time, b: Time) -> Time {
    if a.nsec >= b.nsec {
        Time {
            sec: a.sec - b.sec,
            nsec: a.nsec - b.nsec,
        }
    } else {
        Time {
            sec: a.sec - b.sec - 1,
            nsec: 1_000_000_000 + a.nsec - b.nsec,
        }
    }
}

fn toSec(time: Time) -> f64 {
    time.sec as f64 + time.nsec as f64 / 1_000_000_000.0
}

fn compare(a: Time, b: Time) -> i32 {
    if a.sec > b.sec {
        1
    } else if a.sec < b.sec {
        -1
    } else {
        a.nsec.cmp(&b.nsec)
    }
}
```