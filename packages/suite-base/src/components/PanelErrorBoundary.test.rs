```rust
mod tests {
    use super::*;

    #[test]
    fn test_panel_error_boundary() {
        // Test cases here...
    }
}
```

```rust
fn render_error_boundary(children: ReactNode, props: PanelErrorBoundaryProps) -> RenderResult<ThemeProvider> {
    ThemeProvider::render(
        <PanelErrorBoundary {...props}>{children}</PanelErrorBoundary>,
    )
}

struct PanelErrorBoundaryProps {
    onResetPanel: fn() -> (),
    onRemovePanel: fn() -> (),
    onLogError: fn(message: &str, error: Option<Box<dyn std::error::Error>>),
    showErrorDetails: bool,
    hideErrorSourceLocations: bool,
}
```

```rust
fn main() {
    // Main execution logic here...
}
```