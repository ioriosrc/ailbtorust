```rust
{
  name: String::from("web"),
  private: true,
  devDependencies: std::collections::HashMap::from([
    ("@lichtblick/log", "workspace:*".to_string()),
    ("@lichtblick/suite-web", "workspace:*".to_string()),
    ("@lichtblick/tsconfig", "1.0.2".to_string()),
    ("@types/serve-handler", "^6".to_string()),
    ("playwright", "1.55.1".to_string()),
    ("serve-handler", "6.1.7".to_string()),
    ("webpack", "5.105.4".to_string()),
    ("webpack-dev-server", "5.2.3".to_string())
  ])
}
```