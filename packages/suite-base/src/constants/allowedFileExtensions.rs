```rust
enum AllowedFileExtensions {
    FOXE(".foxe"),
    JSON(".json"),
    BAG(".bag"),
    MCAP(".mcap"),
    DB3(".db3"),
    ULG(".ulg"),
    ULOG(".ulog"),
}

impl AllowedFileExtensions {
    fn from_str(file_extension: &str) -> Option<Self> {
        match file_extension {
            "foxe" => Some(AllowedFileExtensions::FOXE),
            "json" => Some(AllowedFileExtensions::JSON),
            "bag" => Some(AllowedFileExtensions::BAG),
            "mcap" => Some(AllowedFileExtensions::MCAP),
            "db3" => Some(AllowedFileExtensions::DB3),
            "ulg" => Some(AllowedFileExtensions::ULG),
            "ulog" => Some(AllowedFileExtensions::ULOG),
            _ => None,
        }
    }
}

fn main() {
    // Example usage
    let file_extension = "foxe";
    match AllowedFileExtensions::from_str(file_extension) {
        Some(extension) => println!("Valid file extension: {}", extension),
        None => println!("Invalid file extension"),
    }
}
```