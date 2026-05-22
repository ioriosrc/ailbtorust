```rust
use std::env;

fn load_panel_logs_height() -> u32 {
    match env::var("LOCAL_STORAGE_PANEL_LOGS_HEIGHT") {
        Ok(saved) => saved.parse::<u32>().unwrap_or(MAX_HEIGHT),
        Err(_) => MAX_HEIGHT,
    }
}

fn save_panel_logs_height(height: u32) {
    env::set_var("LOCAL_STORAGE_PANEL_LOGS_HEIGHT", height.to_string());
}
```

Esses códigos Rust são equivalentes ao JavaScript anterior, ambos contendo funções para ler e salvar valores do localStorage. A diferença é que no Rust, as variáveis de ambiente são acessadas usando `env::var` em vez de `localStorage.getItem`, e a conversão entre strings e números é realizada manualmente.