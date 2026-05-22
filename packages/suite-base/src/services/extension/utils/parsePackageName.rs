```rust
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct PackageName {
    pub name: String,
    pub publisher: String,
}

impl FromStr for PackageName {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NAME_PATTERN: &str = r"^@([^/]+)\/(.+)$";
        let re = regex::Regex::new(NAME_PATTERN).unwrap();

        if !re.is_match(s) {
            return Err("Invalid package name format");
        }

        Ok(Self {
            name: re.captures(s)?.get(2)?.as_str().to_string(),
            publisher: re.captures(s)?.get(1)?.as_str().to_string(),
        })
    }
}
```