```rust
fn is_desktop_app() -> bool {
    global.get("desktopBridge").map(|bridge| bridge.is_some()).unwrap_or(false)
}
```

Esta função verifica se o ambiente é um aplicativo desktop. No Rust, a maneira mais comum de acessar variáveis globais é usando `global!`.