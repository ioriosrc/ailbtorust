```rust
type SingleLoaderInstallResult =
    | {
        loader_type: TypeExtensionLoader,
        success: true,
        info: ExtensionInfo,
        contribution_points: ContributionPoints,
        external_id: String,
    }
    | {
        loader_type: TypeExtensionLoader,
        success: false,
        error: Box<dyn std::error::Error>,
    };
```