```rust
use std::str;

fn parse_package_name(package_name: &str) -> Result<PackageName, &'static str> {
    if package_name.starts_with('@') {
        let parts: Vec<&str> = package_name[1..].split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid package name format");
        }
        let publisher = parts[0];
        let name = parts[1];

        Ok(PackageName { publisher, name })
    } else {
        Ok(PackageName { publisher: String::from(package_name), name: String::from(package_name) })
    }
}

#[derive(Debug)]
pub struct PackageName {
    pub publisher: String,
    pub name: String,
}
```