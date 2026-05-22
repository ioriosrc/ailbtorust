```rust
use std::cmp::{Ord, PartialOrd};
use std::pin::Pin;

use heap_vec::HeapVec;

pub struct SequentialIteratorMergeOptions<T> {
    value: T,
    iterator: Pin<Box<dyn Iterator<Item = T>>>,
}

impl<T: Ord + PartialOrd + Send> Ord for SequentialIteratorMergeOptions<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T: Ord + PartialOrd + Send> PartialEq for SequentialIteratorMergeOptions<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

pub struct SourceWithTime {
    source: IIterableSource,
    start_time: std::time::SystemTime,
    end_time: Option<std::time::SystemTime>,
}

pub async fn merge_sequential_iterators<T>(
    sources: Vec<IIterableSource>,
    args: MessageIteratorArgs,
) -> impl Iterator<Item = T> {
    let mut sources_with_time: Vec<SourceWithTime> = vec![];
    let mut sources_without_time: Vec<&dyn IIterableSource> = vec![];

    for source in sources.into_iter() {
        let start_time = match source.start() {
            Some(time) => time,
            None => continue,
        };
        let end_time = match source.end() {
            Some(time) => time,
            None => continue,
        };
        sources_with_time.push(SourceWithTime {
            source,
            start_time,
            end_time: Some(end_time),
        });
    }

    let mut heap = HeapVec::new(|a, b| compare(&a.value, &b.value));

    async fn activate_source(source: IIterableSource) -> Option<T> {
        let iterator = Box::pin(source.message_iterator(args));
        let result = iterator.next();
        if let Some(next_result) = result {
            Some((next_result.msg_event.receive_time.into(), next_result))
        } else {
            None
        }
    }

    for source in sources_without_time.into_iter() {
        activate_source(*source).await;
    }

    let mut next_source_index = 0;

    async fn activate_next_source() -> Option<T> {
        if let Some((timestamp, result)) = heap.pop() {
            let current_time_ms = timestamp.timestamp_millis();
            while next_source_index < sources_with_time.len()
                && sources_with_time[next_source_index].start_time <= current_time_ms
            {
                let source_info = &sources_with_time[next_source_index];
                activate_source(source_info.source).await;
                next_source_index += 1;
            }
            Some(result)
        } else {
            None
        }
    }

    if args.start.is_some() {
        while !heap.is_empty() && sources_with_time[next_source_index].start_time > args.start.unwrap().timestamp_millis() {
            heap.pop();
        }
    }

    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        let timestamp = node.value.timestamp_millis();

        if next_source_index < sources_with_time.len()
            && sources_with_time[next_source_index].start_time <= timestamp
        {
            activate_next_source().await;
        }

        yield node.value;

        let next_result = node.iterator.next();
        if let Some((timestamp, result)) = next_result {
            heap.push(SequentialIteratorMergeOptions {
                value: result,
                iterator: Pin::from(Box::new(iterator)),
            });
        } else {
            if heap.is_empty() && next_source_index < sources_with_time.len() {
                activate_next_source().await;
            }
        }
    }

    // Close all active iterators to release resources (e.g. HTTP connections)
    for node in heap.into_iter() {
        let iterator = node.iterator.take();
        if let Err(e) = iterator.map(|mut iter| iter.return()).await {
            eprintln!("Error closing iterator: {}", e);
        }
    }
}

fn get_time(event: IteratorResult): u64 {
    if event.type == "message-event" {
        return event.msg_event.receive_time.timestamp_millis();
    } else if event.type == "stamp" {
        return event.stamp.timestamp_millis();
    }
    return std::u64::MAX;
}
```