```rust
// Rust code snippet to convert the given TypeScript/React Jest configuration to a functional style

struct JestConfig {
    global_setup: String,
    transform: Vec<String>,
    "//": String,
    haste: HasteConfig,
}

struct HasteConfig {
    force_node_filesystem_api: bool,
}

impl Default for JestConfig {
    fn default() -> Self {
        Self {
            global_setup: "../../integration-test-build.ts".to_string(),
            transform: vec!["babel-jest".to_string(), { "rootMode": "upward" }],
            "//": "// Native find is slow because it does not exclude files: https://github.com/facebook/jest/pull/11264#issuecomment-825377579".to_string(),
            haste: HasteConfig {
                force_node_filesystem_api: true,
            },
        }
    }
}

fn main() {
    // Example usage
    let config = JestConfig::default();
    println!("Global Setup: {}", config.global_setup);
    println!("Transforms: {:?}", config.transform);
    println!("Comment: {}", config.comment);
    println!("Haste Config: {:?}", config.haste);
}
```