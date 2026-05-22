```rust
fn has_transformer_errors(script_data: &ScriptData) -> bool {
    script_data.diagnostics.iter().any(|diagnostic| diagnostic.severity == DIAGNOSTIC_SEVERITY::Error)
}
```