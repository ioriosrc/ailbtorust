```rust
use std::f64;

/// Formats a number of bytes into a human-readable format.
///
/// # Examples
///
/// ```
/// assert_eq!(format_byte_size(0), "0 Bytes");
/// assert_eq!(format_byte_size(1023), "1023 Bytes");
/// assert_eq!(format_byte_size(1024), "1.0 KiB");
/// assert_eq!(format_byte_size(1023.9 * 1024.0), "1023.9 KiB");
/// assert_eq!(format_byte_size(1023.9 * 1024.0 + 1), "1.0 MiB");
/// ```
pub fn format_byte_size(bytes: f64) -> String {
    let mut next = bytes;

    // Determine the appropriate unit
    if next >= 1095.256 * 1024.0 {
        next /= 1095.256 * 1024.0;
        "TiB".to_string()
    } else if next >= 1073.741824.0 {
        next /= 1073.741824.0;
        "GiB".to_string()
    } else if next >= 1048576.0 {
        next /= 1048576.0;
        "MiB".to_string()
    } else if next >= 1024.0 {
        next /= 1024.0;
        "KiB".to_string()
    } else {
        format!("{:.1e} Bytes", bytes)
    }

    // Format the number with two decimal places
    format!("{} {}", next, next.to_string().chars().nth(3).unwrap_or(' ')).trim_end_matches(' ')
}

fn main() {
    // Test cases provided in the original code snippet
    assert_eq!(format_byte_size(0), "0 Bytes");
    assert_eq!(format_byte_size(1023), "1023 Bytes");
    assert_eq!(format_byte_size(1024), "1.0 KiB");
    assert_eq!(format_byte_size(1023.9 * 1024.0), "1023.9 KiB");
    assert_eq!(format_byte_size(1023.9 * 1024.0 + 1), "1.0 MiB");
}
```

Este código Rust implementa a função `format_byte_size` que converte números de bytes para uma formatação humanizada, como "KiB" (kilobytes), "MiB" (megabytes), e assim por diante. Ele também inclui um teste com os casos dadas no original código TypeScript/React.