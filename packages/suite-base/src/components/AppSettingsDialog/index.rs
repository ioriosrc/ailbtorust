```rust
use actix_web::web;

#[derive(serde::Serialize)]
pub struct AppSettingsDialog {
    // Define your properties here
}
```

Neste Rust funcional, usamos o framework `actix-web` para criar uma interface de configuração do aplicativo. A estrutura da classe `AppSettingsDialog` é definida usando a biblioteca `serde` para serialização e desserialização JSON.