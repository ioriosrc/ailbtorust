```rust
use crate::suite_base::{services::extension::utils::parse_package_name, types::Extensions};

pub fn validate_package_info(info: Partial<Extensions>) -> Extensions {
    if let Some(name) = info.name {
        if name.is_empty() {
            panic!("Invalid extension: missing name");
        }
        let parsed_publisher = parse_package_name(&name);
        let publisher = info.publisher.unwrap_or(parsed_publisher);
        if publisher.is_empty() {
            panic!("Invalid extension: missing publisher");
        }

        return Extensions {
            ..info,
            publisher,
            name.to_lowercase(),
        };
    }

    info
}
```