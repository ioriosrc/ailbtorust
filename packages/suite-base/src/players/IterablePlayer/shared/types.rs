```rust
use crate::rostime::Time;
use crate::suite_base::players::IterablePlayer::IIterableSource;

pub enum MultiSource {
    Files(Vec<Blob>),
    Urls(Vec<String>, Option<usize>),
}

pub type IterableSourceConstructor<T extends IIterableSource, P> = new (args: P) -> T;

pub struct InitMetadata {}

pub type InitTopicStatsMap = Initialization["topic_stats"];

pub type SourceWithTime {
    source: IIterableSource,
    startTime: Time,
    endTime: Time,
}

pub type SequentialIteratorMergeOptions<T extends IteratorResult> = {
    value: T,
    iterator: AsyncIterableIterator<Readonly<IteratorResult>>,
};
```