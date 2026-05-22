```rust
pub const COMPRESSED_POINTER_SIZE: usize = 4; // Pointers use 4 bytes (also on 64-bit systems) due to pointer compression
pub const OBJECT_BASE_SIZE: usize = 3 * COMPRESSED_POINTER_SIZE; // 3 compressed pointers
// Arrays have an additional length property (1 pointer) and a backing store header (2 pointers)
// See https://stackoverflow.com/a/70550693.
pub const ARRAY_BASE_SIZE: usize = OBJECT_BASE_SIZE + 3 * COMPRESSED_POINTER_SIZE;
pub const TYPED_ARRAY_BASE_SIZE: usize = 25 * COMPRESSED_POINTER_SIZE; // byteLength, byteOffset, ..., see https://stackoverflow.com/a/45808835
pub const SMALL_INTEGER_SIZE: usize = COMPRESSED_POINTER_SIZE; // Small integers (up to 31 bits), pointer tagging
pub const HEAP_NUMBER_SIZE: usize = 8 + 2 * COMPRESSED_POINTER_SIZE; // 4-byte map pointer + 8-byte payload + property pointer
pub const FIELD_SIZE_BY_PRIMITIVE: &'static [(String, usize)] = vec![
    ("bool", SMALL_INTEGER_SIZE),
    ("int8", SMALL_INTEGER_SIZE),
    ("uint8", SMALL_INTEGER_SIZE),
    ("int16", SMALL_INTEGER_SIZE),
    ("uint16", SMALL_INTEGER_SIZE),
    ("int32", SMALL_INTEGER_SIZE),
    ("uint32", SMALL_INTEGER_SIZE),
    ("float32", HEAP_NUMBER_SIZE),
    ("float64", HEAP_NUMBER_SIZE),
    ("int64", HEAP_NUMBER_SIZE),
    ("uint64", HEAP_NUMBER_SIZE),
    ("time", OBJECT_BASE_SIZE + 2 * HEAP_NUMBER_SIZE + COMPRESSED_POINTER_SIZE),
    ("duration", OBJECT_BASE_SIZE + 2 * HEAP_NUMBER_SIZE + COMPRESSED_POINTER_SIZE),
    ("string", 20), // we don't know the length upfront, assume a fixed length
].into_iter().collect();
pub const MAX_NUM_FAST_PROPERTIES: usize = 1020;

// Capabilities that are not shared by all players.
pub const PLAYER_CAPABILITIES: &'static [&'static str] = vec![
    "advertise",
    "assets",
    "callServices",
    "setSpeed",
    "playbackControl",
    "getParameters",
    "setParameters",
].into_iter().collect();
```