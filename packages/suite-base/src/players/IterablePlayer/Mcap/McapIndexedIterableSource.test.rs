```rust
use std::fs;
use tempfile::TempFile;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use @mcap/core::{McapIndexedReader, McapWriter, TempBuffer};
use @lichtblick/mcap-support::loadDecompressHandlers;
use @lichtblick/suite-base/players/IterablePlayer/Mcap/BlobReadable;
use @lichtblick/suite-base/players/IterablePlayer/Mcap/McapIndexedIterableSource;

#[test]
async fn mcapi_indexed_iterable_source_returns_correct_metadata() {
    let temp_buffer = TempFile::new().unwrap();
    let mut writer = McapWriter::new_with_start_channel_id(temp_buffer, 1);
    writer.start_with_library_and_profile(&[], &[]).await.unwrap();
    writer.register_schema(
        "test",
        "test".as_bytes(),
        "1",
        "1",
        &[],
        Some(McapIndexedReader::default_metadata()),
    ).unwrap();
    writer.register_channel(
        "1",
        "ros1msg",
        "1",
        "1",
        &["test".to_string()],
        None,
    ).unwrap();
    writer.add_message(
        1,
        0,
        0,
        1,
        vec![],
    ).unwrap();
    writer.add_metadata("metadata1", vec![(b"key".to_vec(), b"value".to_vec())]).await.unwrap();
    writer.end().await.unwrap();

    let readable = BlobReadable::new(temp_buffer.into_inner()).unwrap();
    let decompress_handlers = load_decompress_handlers().await.unwrap();
    let reader = McapIndexedReader::initialize(readable, decompress_handlers).await.unwrap();

    let source = McapIndexedIterableSource::new(reader);
    let metadata = source.initialize().await.unwrap();

    assert!(metadata.is_some());
    assert_eq!(
        metadata.unwrap(),
        vec![McapMetadata {
            name: "metadata1".to_string(),
            metadata: Some(vec![(b"key".to_vec(), b"value".to_vec())]),
        }],
    );
}

#[test]
async fn mcapi_indexed_iterable_source_returns_empty_array_when_no_metadata_is_on_the_file() {
    let temp_buffer = TempFile::new().unwrap();
    let mut writer = McapWriter::new_with_start_channel_id(temp_buffer, 1);
    writer.start_with_library_and_profile(&[], &[]).await.unwrap();
    writer.register_schema(
        "test",
        "test".as_bytes(),
        "1",
        "1",
        &[],
        Some(McapIndexedReader::default_metadata()),
    ).unwrap();
    writer.register_channel(
        "1",
        "ros1msg",
        "1",
        "1",
        &["test".to_string()],
        None,
    ).unwrap();
    writer.add_message(
        1,
        0,
        0,
        1,
        vec![],
    ).unwrap();
    writer.end().await.unwrap();

    let readable = BlobReadable::new(temp_buffer.into_inner()).unwrap();
    let decompress_handlers = load_decompress_handlers().await.unwrap();
    let reader = McapIndexedReader::initialize(readable, decompress_handlers).await.unwrap();

    let source = McapIndexedIterableSource::new(reader);
    let metadata = source.initialize().await.unwrap();

    assert!(metadata.is_some());
    assert_eq!(metadata.unwrap(), vec![]);
}

#[test]
async fn mcapi_indexed_iterable_source_returns_topic_stats_with_num_messages_and_global_start_end_times_separately() {
    let temp_buffer = TempFile::new().unwrap();
    let mut writer = McapWriter::new_with_start_channel_id(temp_buffer, 1);
    writer.start_with_library_and_profile(&[], &[]).await.unwrap();
    writer.register_schema(
        "test",
        "test".as_bytes(),
        "1",
        "1",
        &[],
        Some(McapIndexedReader::default_metadata()),
    ).unwrap();
    writer.register_channel(
        "1",
        "ros1msg",
        "1",
        "1",
        &["test".to_string()],
        None,
    ).unwrap();
    writer.add_message(
        1,
        0,
        0,
        1,
        vec![],
    ).unwrap();
    writer.add_message(
        1,
        5,
        0,
        2,
        vec![],
    ).await();

    let readable = BlobReadable::new(temp_buffer.into_inner()).unwrap();
    let decompress_handlers = load_decompress_handlers().await.unwrap();
    let reader = McapIndexedReader::initialize(readable, decompress_handlers).await.unwrap();

    let source = McapIndexedIterableSource::new(reader);
    let (topic_stats, start, end) = source.initialize().await.unwrap();

    assert!(topic_stats.is_some());
    let topic_stats = topic_stats.unwrap();
    let test_topic_stats = topic_stats.get("test").unwrap();
    assert_eq!(test_topic_stats.num_messages, 2);
    assert!(test_topic_stats.first_message_time.is_none());
    assert!(test_topic_stats.last_message_time.is_none());

    assert!(start.is_some());
    assert_eq!(
        start.unwrap(),
        McapInstant {
            sec: 0,
            nsec: 1_000_000_000,
        }
    );

    assert!(end.is_some());
    assert_eq!(
        end.unwrap(),
        McapInstant {
            sec: 5,
            nsec: 0,
        }
    );
}

#[test]
async fn mcapi_indexed_iterable_source_get_end_returns_undefined_before_initialization() {
    // Given an indexed source that has not been initialized
    let temp_buffer = TempFile::new().unwrap();
    let mut writer = McapWriter::new_with_start_channel_id(temp_buffer, 1);
    writer.start_with_library_and_profile(&[], &[]).await.unwrap();
    writer.register_schema(
        "test",
        "test".as_bytes(),
        "1",
        "1",
        &[],
        Some(McapIndexedReader::default_metadata()),
    ).unwrap();
    writer.register_channel(
        "1",
        "ros1msg",
        "1",
        "1",
        &["test".to_string()],
        None,
    ).unwrap();
    writer.add_message(
        1,
        0,
        0,
        1,
        vec![],
    ).unwrap();
    writer.end().await.unwrap();

    let readable = BlobReadable::new(temp_buffer.into_inner()).unwrap();
    let decompress_handlers = load_decompress_handlers().await.unwrap();
    let reader = McapIndexedReader::initialize(readable, decompress_handlers).await.unwrap();

    let source = McapIndexedIterableSource::new(reader);

    // When calling getEnd before initialize
    // Then it should return undefined
    assert!(source.get_end().is_none());
}

#[test]
async fn mcapi_indexed_iterable_source_get_end_returns_the_latest_message_end_time_after_initialization() {
    // Given an indexed MCAP with messages spanning from 2s to 10s
    let temp_buffer = TempFile::new().unwrap();
    let mut writer = McapWriter::new_with_start_channel_id(temp_buffer, 1);
    writer.start_with_library_and_profile(&[], &[]).await.unwrap();
    writer.register_schema(
        "test",
        "test".as_bytes(),
        "1",
        "1",
        &[],
        Some(McapIndexedReader::default_metadata()),
    ).unwrap();
    writer.register_channel(
        "1",
        "ros1msg",
        "1",
        "1",
        &["test".to_string()],
        None,
    ).unwrap();
    writer.add_message(
        1,
        0,
        0,
        1,
        vec![],
    ).await();
    writer.add_message(
        1,
        5,
        0,
        2,
        vec![],
    ).await();

    let readable = BlobReadable::new(temp_buffer.into_inner()).unwrap();
    let decompress_handlers = load_decompress_handlers().await.unwrap();
    let reader = McapIndexedReader::initialize(readable, decompress_handlers).await.unwrap();

    let source = McapIndexedIterableSource::new(reader);
    await source.initialize().await.unwrap();

    // When initializing and calling getEnd
    let end = source.get_end().await.unwrap();

    assert!(end.is_some());
    assert_eq!(
        end.unwrap(),
        McapInstant {
            sec: 5,
            nsec: 0,
        }
    );
}
```