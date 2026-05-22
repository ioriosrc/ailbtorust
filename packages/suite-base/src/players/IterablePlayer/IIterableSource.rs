```rust
use async_std::prelude::*;
use futures::AsyncReadExt;
use std::time::{Duration, Instant};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

pub type TopicWithDecodingInfo = Topic & {
    message_encoding: Option<String>,
    schema_encoding: Option<String>,
    schema_data: Option<Vec<u8>>,
};

pub type Initialization = {
    start: Instant,
    end: Instant,
    topics: Vec<TopicWithDecodingInfo>,
    topic_stats: std::collections::HashMap<String, TopicStats>,
    datatypes: RosDatatypes,
    profile: Option<String>,
    name: Option<&'static str>,
    metadata: Vec<Metadata>,

    /** Publisher names by topic **/
    publishers_by_topic: std::collections::HashMap<String, std::collections::HashSet<String>>,

    alerts: Vec<PlayerAlert>,
};

pub type MessageIteratorArgs = {
    topics: TopicSelection;

    /**
     * The start time of the iterator (inclusive). If no start time is specified, the iterator will start
     * from the beginning of the source.
     *
     * The first message receive_time will be >= start.
     */
    start: Option<Instant>;

    /**
     * The end time of the iterator (inclusive). If no end time is specified, the iterator will stop
     * at the end of the source.
     *
     * The last message receive_time will be <= end.
     */
    end: Option<Instant>;

    /**
     * Indicate the expected way the iterator is consumed.
     *
     * Data sources may choose to change internal mechanics depending on whether the messages are
     * consumed immediate in full from the iterator or if it might be read partially.
     *
     * `full` indicates that the caller plans to read the entire iterator
     * `partial` indicates that the caller plans to read the iterator but may not read all the messages
     */
    consumption_type: "full" | "partial";
};

/**
 * IteratorResult represents a single result from a message iterator or cursor. There are three
 * types of results.
 *
 * - message-event: the result contains a MessageEvent
 * - alert: the result contains an alert
 * - stamp: the result is a timestamp
 *
 * Note: A stamp result acts as a marker indicating that the source has reached the specified stamp.
 * The source may return stamp results to indicate to callers that it has read through some time
 * when there are no message events available to indicate the time is reached.
 */
pub type IteratorResult<MessageType = unknown> =
    | {
        type: "message-event";
        msg_event: MessageEvent<MessageType>;
    }
    | {
        type: "alert";
        /**
         * An ID representing the channel/connection where this alert came from. The app may choose
         * to display only a single alert from each connection to avoid overwhelming the user.
         */
        connection_id: u32;
        alert: PlayerAlert;
    }
    | {
        type: "stamp";
        stamp: Instant;
    };

pub type GetBackfillMessagesArgs = {
    topics: TopicSelection;
    time: Instant;

    abort_signal: Option<AbortSignal>;
};

// IMessageCursor describes an interface for message cursors. Message cursors are a similar concept
// to javascript generators but provide a method for reading a batch of messages rather than one
// message.
//
// Motivation: When using webworkers, read calls are invoked via an RPC interface. For large
// datasets (many hundred thousand) messages, preloading the data (i.e. to plot a signal) would
// result in several hundred thousand RPC calls. The overhead of making these calls add up and
// negatively impact the preloading experience.
//
// Providing an interface which allows callers to read a batch of messages significantly (4x speedup
// on an 700k message dataset on M1 Pro) reduces the RPC call overhead.
pub trait IMessageCursor<MessageType = unknown> {
    /**
     * Read the next message from the cursor. Return a result or undefined if the cursor is done
     */
    async fn next(&mut self) -> Option<IteratorResult<MessageType>>;

    /**
     * Read the next batch of messages from the cursor. Return an array of results or undefined if the cursor is done.
     *
     * @param duration_ms indicate the duration (in milliseconds) for the batch to stop waiting for
     * more messages and return. This duration tracks the receive time from the first message in the
     * batch.
     */
    async fn next_batch(&mut self, duration_ms: u32) -> Option<Vec<IteratorResult<MessageType>>>;

    /**
     * Read a batch of messages through end time (inclusive) or end of cursor
     *
     * return undefined when no more message remain in the cursor
     */
    async fn read_until(&mut self, end: Instant) -> Option<Vec<IteratorResult<MessageType>>>;

    /**
     * End the cursor
     *
     * Release any held resources by the cursor.
     *
     * Calls to next() and readUntil() should return `undefined` after a cursor is ended as if the
     * cursor reached the end of its messages.
     */
    async fn end(&mut self) -> Option<()>;
}

/**
 * IIterableSource specifies an interface for initializing and accessing messages using iterators.
 *
 * IIterableSources also provide a backfill method to obtain the last message available for topics.
 */
pub trait IIterableSource<MessageType = unknown> {
    /**
     * Initialize the source.
     */
    async fn initialize(&mut self) -> Initialization;

    /**
     * Instantiate an IMessageIterator for the source.
     *
     * The iterator produces IteratorResults from the source. The IteratorResults should be in log
     * time order.
     *
     * Returning an AsyncIterator rather than AsyncIterable communicates that the returned iterator
     * cannot be used directly in a `for-await-of` loop. This forces the IterablePlayer implementation
     * to use the `.next()` API, rather than `for-await-of` which would implicitly call the iterator's
     * `return()` method when breaking out of the loop and prevent the iterator from being used in
     * more than one loop. This means the IIterableSource implementations can use a simple async
     * generator function, and a `finally` block to do any necessary cleanup tasks when the request
     * finishes or is canceled.
     */
    async fn message_iterator(
        &mut self,
        args: Immutable<MessageIteratorArgs>,
    ) -> AsyncIterator<Readonly<IteratorResult<MessageType>>>;

    /**
     * Load the most recent messages per topic that occurred before or at the target time, if
     * available.
     */
    async fn get_backfill_messages(&mut self, args: Immutable<GetBackfillMessagesArgs>) -> Vec<Immutable<MessageEvent<MessageType>>>;

    /**
     * A source can optionally implement a cursor interface in addition to a messageIterator interface.
     *
     * A cursor interface provides methods to read messages in batches rather than one at a time.
     * This improves performance for some workflows (i.e. message reading over webworkers) by avoiding
     * individual "next" calls per message.
     */
    fn get_message_cursor(&mut self) -> Option<IMessageCursor<MessageType>>;

    fn get_start(&self) -> Option<Instant>;

    fn get_end(&self) -> Option<Instant>;

    /**
     * Optional method a data source can implement to cleanup resources. The player will call this
     * method when the source will no longer be used.
     */
    async fn terminate(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub type IterableSourceInitializeArgs = {
    file: Option<File>;
    url: Option<String>;
    files: Option<Vec<File>>;
    urls: Option<Vec<String>>;
    params: Option<Record<std::string, String>>;
};

/**
 * Interface for a raw iterable source where messages are in their serialized byte form (Uint8Arrays).
 * A raw source is well suited for workers as array buffers can be efficientely transferred to the main thread.
 */
pub type ISerializedIterableSource = IIterableSource<u8> & { source_type: "serialized" };

/**
 * Interface for a deserialized iterable source where messages are in their deserialized form (unknown).
 */
pub type IDeserializedIterableSource = IIterableSource<unknown>;
```