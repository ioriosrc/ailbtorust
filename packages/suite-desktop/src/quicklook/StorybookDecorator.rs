```rust
fn storybook_decorator(child: impl Fn()) -> impl Fn() {
    move || {
        let global_style = GlobalStyle {};
        child();
    }
}
```

No Rust, as funções podem ser definidas como funções retornando outro função. Isso é muito semelhante ao comportamento do TypeScript/React `StorybookDecorator`.