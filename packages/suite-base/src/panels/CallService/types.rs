```rust
struct Config {
    service_name: Option<String>,
    request_payload: Option<String>,
    layout: String,
    button_text: String,
    button_tooltip: String,
    button_color: String,
}

impl Config {
    fn new(service_name: Option<String>, request_payload: Option<String>, layout: &str, button_text: &str, button_tooltip: &str, button_color: &str) -> Self {
        Config {
            service_name,
            request_payload,
            layout: layout.to_string(),
            button_text: button_text.to_string(),
            button_tooltip: button_tooltip.to_string(),
            button_color: button_color.to_string(),
        }
    }
}
```

Esta implementação corresponde ao código TypeScript/React fornecido. Ela cria uma struct Rust `Config` que tem o mesmo layout e os tipos de dados da struct original, mas altera os métodos para criar um objeto `Config`.