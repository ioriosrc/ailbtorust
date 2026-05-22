```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JestConfig {
    projects: Vec<String>,
    coverage_directory: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = JestConfig {
        projects: vec![
            "<rootDir>/ci/jest.config.json",
            "<rootDir>/desktop/jest.config.json",
            "<rootDir>/packages/*/jest.config.json"
        ],
        coverage_directory: "coverage".to_string(),
    };

    println!("{:?}", config);

    Ok(())
}
```