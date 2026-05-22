```rust
struct ExtensionPackageJson {
    name: String,
    version: String,
    main: String,
    publisher: Option<String>,
}

struct PackageName {
    name: String,
    namespace: Option<String>,
}
```