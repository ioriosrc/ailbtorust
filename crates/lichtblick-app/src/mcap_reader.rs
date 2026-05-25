// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! MCAP format reader with summary-first, chunk-on-demand architecture.
//! Reads only the file footer/summary for instant open, then loads chunks lazily.

use std::collections::HashMap;
use std::rc::Rc;

/// MCAP record opcodes.
const OP_HEADER: u8 = 0x01;
const OP_FOOTER: u8 = 0x02;
const OP_SCHEMA: u8 = 0x03;
const OP_CHANNEL: u8 = 0x04;
const OP_MESSAGE: u8 = 0x05;
const OP_CHUNK: u8 = 0x06;
const OP_MESSAGE_INDEX: u8 = 0x07;
const OP_CHUNK_INDEX: u8 = 0x08;
const OP_ATTACHMENT: u8 = 0x09;
const OP_ATTACHMENT_INDEX: u8 = 0x0A;
const OP_STATISTICS: u8 = 0x0B;
const OP_METADATA: u8 = 0x0C;
const OP_METADATA_INDEX: u8 = 0x0D;
const OP_SUMMARY_OFFSET: u8 = 0x0E;
const OP_DATA_END: u8 = 0x0F;

const MCAP_MAGIC: &[u8] = b"\x89MCAP0\r\n";

/// Schema info from the summary.
#[derive(Clone, Debug)]
pub struct SchemaInfo {
    pub id: u16,
    pub name: String,
    pub encoding: String,
    /// Raw schema data (e.g. ROS .msg text) for field-level decoding.
    pub data: Vec<u8>,
}

/// Channel info from the summary.
#[derive(Clone, Debug)]
pub struct ChannelInfo {
    pub id: u16,
    pub schema_id: u16,
    pub topic: String,
    pub message_encoding: String,
}

/// Chunk index entry - tells us where chunks are and what time they cover.
#[derive(Clone, Debug)]
pub struct ChunkIndexEntry {
    pub message_start_time: u64,
    pub message_end_time: u64,
    pub chunk_offset: u64,
    pub chunk_length: u64,
    pub message_index_offset: u64,
    pub message_index_length: u64,
    pub compression: String,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}

/// Statistics from the summary.
#[derive(Clone, Debug, Default)]
pub struct McapStatistics {
    pub message_count: u64,
    pub schema_count: u16,
    pub channel_count: u32,
    pub chunk_count: u32,
    pub message_start_time: u64,
    pub message_end_time: u64,
    /// Per-channel message counts from the Statistics record.
    pub channel_message_counts: HashMap<u16, u64>,
}

/// Full summary parsed from the MCAP footer.
#[derive(Clone, Debug)]
pub struct McapSummary {
    pub schemas: HashMap<u16, SchemaInfo>,
    pub channels: HashMap<u16, ChannelInfo>,
    pub chunk_indices: Vec<ChunkIndexEntry>,
    pub statistics: McapStatistics,
}

/// A decoded message from a chunk.
#[derive(Clone, Debug)]
pub struct DecodedMessage {
    pub channel_id: u16,
    pub log_time: u64,
    pub publish_time: u64,
    pub data: Rc<Vec<u8>>,
}

/// Read MCAP summary from file footer.
/// Only needs the last portion of the file (from summary_start to end).
/// `footer_data` is the last N bytes of the file, `footer_offset` is where those bytes start in the file.
pub fn read_summary_from_end(data: &[u8]) -> Result<(McapSummary, u64), String> {
    let len = data.len();
    if len < 8 + 9 + 8 {
        return Err("File too small to be valid MCAP".into());
    }

    // Verify trailing magic
    let trailing_magic = &data[len - 8..];
    if trailing_magic != MCAP_MAGIC {
        return Err(format!("Invalid MCAP trailing magic: {:?}", &trailing_magic[..4]));
    }

    // Footer record is just before the trailing magic.
    // Record format: [opcode:1][length:8][data:length]
    // Footer data is 20 bytes: summary_start(8) + summary_offset_start(8) + summary_crc(4)
    // So footer record is: 1 + 8 + 20 = 29 bytes before trailing magic.
    let footer_record_end = len - 8;
    let footer_record_start = footer_record_end - 29;

    if footer_record_start >= len {
        return Err("Cannot find footer record".into());
    }

    let opcode = data[footer_record_start];
    if opcode != OP_FOOTER {
        return Err(format!("Expected footer opcode 0x02, got 0x{:02x}", opcode));
    }

    let record_len = read_u64_le(&data[footer_record_start + 1..]) as usize;
    let footer_data_start = footer_record_start + 9;

    if footer_data_start + record_len > footer_record_end {
        return Err("Footer record extends past end of file".into());
    }

    let footer_bytes = &data[footer_data_start..footer_data_start + record_len];
    let summary_start = read_u64_le(footer_bytes);
    let _summary_offset_start = read_u64_le(&footer_bytes[8..]);

    if summary_start == 0 {
        return Err("MCAP file has no summary section (summary_start=0). File may need re-indexing.".into());
    }

    // Try to parse summary from our buffer.
    // Our buffer may be a tail slice of the file. summary_start is an absolute file offset.
    // We need to know what absolute offset our buffer starts at.
    // Return the summary_start so the caller can do a targeted read if needed.
    Err(format!("NEED_SUMMARY_READ:{}", summary_start))
}

/// Extract just the summary_start offset from the file footer.
/// Needs only the last ~64 bytes of the file.
pub fn get_summary_start_from_footer(tail: &[u8]) -> Result<u64, String> {
    let len = tail.len();
    if len < 37 {
        return Err("Tail too small to contain footer".into());
    }

    // Verify trailing magic (last 8 bytes)
    let trailing_magic = &tail[len - 8..];
    if trailing_magic != MCAP_MAGIC {
        return Err(format!("Invalid MCAP trailing magic: {:02x?}", &trailing_magic[..4]));
    }

    // Footer is 29 bytes before trailing magic: opcode(1) + length(8) + data(20)
    let footer_start = len - 8 - 29;
    let opcode = tail[footer_start];
    if opcode != OP_FOOTER {
        return Err(format!("Expected footer opcode 0x02, got 0x{:02x}", opcode));
    }

    let footer_data_start = footer_start + 9;
    let summary_start = read_u64_le(&tail[footer_data_start..]);

    if summary_start == 0 {
        return Err("MCAP file has no summary section (summary_start=0)".into());
    }

    Ok(summary_start)
}

/// Parse summary records from data that starts at the summary_start offset.
/// `data` should contain everything from summary_start to the footer (exclusive).
pub fn parse_summary_section(data: &[u8]) -> Result<McapSummary, String> {
    let mut summary = McapSummary {
        schemas: HashMap::new(),
        channels: HashMap::new(),
        chunk_indices: Vec::new(),
        statistics: McapStatistics::default(),
    };

    let len = data.len();
    let mut pos = 0;

    while pos + 9 <= len {
        let op = data[pos];
        let rec_len = read_u64_le(&data[pos + 1..]) as usize;
        let rec_data_start = pos + 9;
        let rec_data_end = rec_data_start + rec_len;

        if rec_data_end > len {
            break;
        }

        // Stop if we hit the footer record
        if op == OP_FOOTER {
            break;
        }

        let rec_data = &data[rec_data_start..rec_data_end];

        match op {
            OP_SCHEMA => {
                if let Some(schema) = parse_schema_record(rec_data) {
                    summary.schemas.insert(schema.id, schema);
                }
            }
            OP_CHANNEL => {
                if let Some(channel) = parse_channel_record(rec_data) {
                    summary.channels.insert(channel.id, channel);
                }
            }
            OP_CHUNK_INDEX => {
                if let Some(ci) = parse_chunk_index_record(rec_data) {
                    summary.chunk_indices.push(ci);
                }
            }
            OP_STATISTICS => {
                if let Some(stats) = parse_statistics_record(rec_data) {
                    summary.statistics = stats;
                }
            }
            _ => {} // Skip other summary records
        }

        pos = rec_data_end;
    }

    // Sort chunk indices by start time
    summary.chunk_indices.sort_by_key(|c| c.message_start_time);

    // If statistics didn't give us times, compute from chunk indices
    if summary.statistics.message_start_time == 0 && !summary.chunk_indices.is_empty() {
        summary.statistics.message_start_time = summary.chunk_indices.first().unwrap().message_start_time;
        summary.statistics.message_end_time = summary.chunk_indices.last().unwrap().message_end_time;
    }

    if summary.channels.is_empty() {
        return Err("No channels found in summary section".into());
    }

    Ok(summary)
}

/// Parse messages from a raw chunk record (the full record including opcode+length has been stripped).
/// `chunk_record_data` is the chunk record's payload (after opcode+length).
pub fn parse_chunk_messages(
    chunk_record_data: &[u8],
    channels: &HashMap<u16, ChannelInfo>,
) -> Result<Vec<DecodedMessage>, String> {
    // Chunk record format:
    // message_start_time: u64
    // message_end_time: u64
    // uncompressed_size: u64
    // uncompressed_crc: u32
    // compression: prefixed string (u32 len + bytes)
    // records_size: u64
    // records: bytes

    if chunk_record_data.len() < 28 {
        return Err("Chunk record too short".into());
    }

    let mut pos = 0;
    let _msg_start = read_u64_le(&chunk_record_data[pos..]); pos += 8;
    let _msg_end = read_u64_le(&chunk_record_data[pos..]); pos += 8;
    let uncompressed_size = read_u64_le(&chunk_record_data[pos..]) as usize; pos += 8;
    let _crc = read_u32_le(&chunk_record_data[pos..]); pos += 4;

    // Read compression string
    let comp_len = read_u32_le(&chunk_record_data[pos..]) as usize; pos += 4;
    if pos + comp_len > chunk_record_data.len() {
        return Err("Invalid compression string length".into());
    }
    let compression = std::str::from_utf8(&chunk_record_data[pos..pos + comp_len])
        .unwrap_or("")
        .to_string();
    pos += comp_len;

    // Read records_size and compressed data
    let records_size = read_u64_le(&chunk_record_data[pos..]) as usize; pos += 8;
    let compressed_data = &chunk_record_data[pos..pos + records_size.min(chunk_record_data.len() - pos)];

    // Decompress
    let decompressed = decompress_chunk(&compression, compressed_data, uncompressed_size)?;

    // Parse messages from decompressed data
    let mut messages = Vec::new();
    let mut rpos = 0;
    let dlen = decompressed.len();

    while rpos + 9 <= dlen {
        let op = decompressed[rpos];
        let rec_len = read_u64_le(&decompressed[rpos + 1..]) as usize;
        let rec_start = rpos + 9;
        let rec_end = rec_start + rec_len;

        if rec_end > dlen {
            break;
        }

        if op == OP_MESSAGE && rec_len >= 22 {
            let msg_data = &decompressed[rec_start..rec_end];
            let channel_id = read_u16_le(msg_data);
            let _sequence = read_u32_le(&msg_data[2..]);
            let log_time = read_u64_le(&msg_data[6..]);
            let publish_time = read_u64_le(&msg_data[14..]);
            let payload = &msg_data[22..];

            // Only store if we know this channel
            if channels.contains_key(&channel_id) {
                messages.push(DecodedMessage {
                    channel_id,
                    log_time,
                    publish_time,
                    data: Rc::new(payload.to_vec()),
                });
            }
        }

        rpos = rec_end;
    }

    Ok(messages)
}

/// Decompress chunk data based on compression type.
fn decompress_chunk(compression: &str, data: &[u8], uncompressed_size: usize) -> Result<Vec<u8>, String> {
    match compression {
        "" | "none" => Ok(data.to_vec()),
        "lz4" => decompress_lz4(data, uncompressed_size),
        "zstd" => decompress_zstd(data, uncompressed_size),
        other => Err(format!("Unsupported compression: {}", other)),
    }
}

fn decompress_lz4(data: &[u8], uncompressed_size: usize) -> Result<Vec<u8>, String> {
    // MCAP uses LZ4 frame format. lz4_flex supports block decompression.
    // Try block decompression first (most common in MCAP).
    lz4_flex::decompress(data, uncompressed_size)
        .map_err(|e| format!("LZ4 decompression failed: {}", e))
}

fn decompress_zstd(data: &[u8], _uncompressed_size: usize) -> Result<Vec<u8>, String> {
    // Use ruzstd (pure Rust zstd decoder, works in WASM)
    let mut decoder = ruzstd::StreamingDecoder::new(data)
        .map_err(|e| format!("Zstd init failed: {}", e))?;
    let mut output = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut output)
        .map_err(|e| format!("Zstd decompression failed: {}", e))?;
    Ok(output)
}

// --- Record parsers ---

fn parse_schema_record(data: &[u8]) -> Option<SchemaInfo> {
    if data.len() < 2 {
        return None;
    }
    let id = read_u16_le(data);
    let mut pos = 2;

    let name = read_prefixed_string(data, &mut pos)?;
    let encoding = read_prefixed_string(data, &mut pos)?;
    // Read schema_data (u32 len + bytes)
    let schema_data = if pos + 4 <= data.len() {
        let len = read_u32_le(&data[pos..]) as usize;
        pos += 4;
        if pos + len <= data.len() {
            data[pos..pos + len].to_vec()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    Some(SchemaInfo { id, name, encoding, data: schema_data })
}

fn parse_channel_record(data: &[u8]) -> Option<ChannelInfo> {
    if data.len() < 4 {
        return None;
    }
    let id = read_u16_le(data);
    let schema_id = read_u16_le(&data[2..]);
    let mut pos = 4;

    let topic = read_prefixed_string(data, &mut pos)?;
    let message_encoding = read_prefixed_string(data, &mut pos)?;
    // Skip metadata (key-value pairs)

    Some(ChannelInfo {
        id,
        schema_id,
        topic,
        message_encoding,
    })
}

fn parse_chunk_index_record(data: &[u8]) -> Option<ChunkIndexEntry> {
    if data.len() < 72 {
        return None;
    }
    let mut pos = 0;
    let message_start_time = read_u64_le(&data[pos..]); pos += 8;
    let message_end_time = read_u64_le(&data[pos..]); pos += 8;
    let chunk_offset = read_u64_le(&data[pos..]); pos += 8;
    let chunk_length = read_u64_le(&data[pos..]); pos += 8;
    let message_index_offset = read_u64_le(&data[pos..]); pos += 8;
    let message_index_length = read_u64_le(&data[pos..]); pos += 8;

    let compression = read_prefixed_string(data, &mut pos).unwrap_or_default();

    let compressed_size = if pos + 8 <= data.len() {
        let v = read_u64_le(&data[pos..]); pos += 8; v
    } else { 0 };

    let uncompressed_size = if pos + 8 <= data.len() {
        read_u64_le(&data[pos..])
    } else { 0 };

    Some(ChunkIndexEntry {
        message_start_time,
        message_end_time,
        chunk_offset,
        chunk_length,
        message_index_offset,
        message_index_length,
        compression,
        compressed_size,
        uncompressed_size,
    })
}

fn parse_statistics_record(data: &[u8]) -> Option<McapStatistics> {
    if data.len() < 36 {
        return None;
    }
    let mut pos = 0;
    let message_count = read_u64_le(&data[pos..]); pos += 8;
    let schema_count = read_u16_le(&data[pos..]); pos += 2;
    let channel_count = read_u32_le(&data[pos..]); pos += 4;
    // attachment_count, metadata_count
    pos += 8; // attachment_count(4) + metadata_count(4)
    let chunk_count = read_u32_le(&data[pos..]); pos += 4;
    let message_start_time = read_u64_le(&data[pos..]); pos += 8;
    let message_end_time = read_u64_le(&data[pos..]); pos += 8;

    // Parse channel_message_counts: length-prefixed array of (channel_id: u16, count: u64)
    let mut channel_message_counts = HashMap::new();
    if pos + 4 <= data.len() {
        let map_len = read_u32_le(&data[pos..]) as usize; pos += 4;
        let map_end = pos + map_len;
        while pos + 10 <= map_end && pos + 10 <= data.len() {
            let ch_id = read_u16_le(&data[pos..]); pos += 2;
            let count = read_u64_le(&data[pos..]); pos += 8;
            channel_message_counts.insert(ch_id, count);
        }
    }

    Some(McapStatistics {
        message_count,
        schema_count,
        channel_count,
        chunk_count,
        message_start_time,
        message_end_time,
        channel_message_counts,
    })
}

// --- Binary reading helpers ---

fn read_u16_le(data: &[u8]) -> u16 {
    u16::from_le_bytes([data[0], data[1]])
}

fn read_u32_le(data: &[u8]) -> u32 {
    u32::from_le_bytes([data[0], data[1], data[2], data[3]])
}

fn read_u64_le(data: &[u8]) -> u64 {
    u64::from_le_bytes([
        data[0], data[1], data[2], data[3],
        data[4], data[5], data[6], data[7],
    ])
}

fn read_prefixed_string(data: &[u8], pos: &mut usize) -> Option<String> {
    if *pos + 4 > data.len() {
        return None;
    }
    let len = read_u32_le(&data[*pos..]) as usize;
    *pos += 4;
    if *pos + len > data.len() {
        return None;
    }
    let s = std::str::from_utf8(&data[*pos..*pos + len]).ok()?.to_string();
    *pos += len;
    Some(s)
}

/// Find which chunk indices overlap a given time range.
pub fn find_chunks_for_time(chunk_indices: &[ChunkIndexEntry], start_ns: u64, end_ns: u64) -> Vec<usize> {
    chunk_indices
        .iter()
        .enumerate()
        .filter(|(_, ci)| ci.message_start_time <= end_ns && ci.message_end_time >= start_ns)
        .map(|(i, _)| i)
        .collect()
}
