```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn uninstall_button(props: ExtensionActionButtonProps) -> Option<Element> {
    Some(jsx!(
        <ExtensionActionButton {...props} color="inherit" />
    ))
}
```
Note: This code snippet assumes that you have a Rust environment setup and the `@lichtblick/suite-base` crate is correctly installed. The JSX syntax used in Rust for rendering HTML elements should be replaced with actual DOM manipulation or React components if working in a web environment.