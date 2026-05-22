```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Layout {
    from: String,
    name: String,
    data: serde_json::Value,
}

impl Layout {
    fn new(from: &str, name: &str, data: serde_json::Value) -> Self {
        Self {
            from: from.to_string(),
            name: name.to_string(),
            data,
        }
    }
}

pub struct DesktopLayoutLoader {
    bridge: Arc<Mutex<dyn Bridge>>,
}

impl DesktopLayoutLoader {
    pub fn new(bridge: Arc<Mutex<dyn Bridge>>) -> Self {
        Self { bridge }
    }

    pub async fn fetch_layouts(&self) -> Result<Vec<Layout>, Box<dyn std::error::Error>> {
        let layouts = self.bridge.lock().unwrap().fetch_layouts()?;
        Ok(layouts.into_iter()
            .map(|layout| Layout::new(&layout.from, &layout.name, serde_json::from_str(&layout.data)?))
            .collect())
    }
}

trait Bridge {
    fn fetch_layouts(&self) -> Result<Vec<Layout>, Box<dyn std::error::Error>>;
}
```

Neste código Rust, estamos implementando um gerenciador de layouts para uma aplicação React. A classe `DesktopLayoutLoader` recebe como parâmetro uma referência ao bridge (um objeto que representa a interface com o ambiente do usuário). O método `fetch_layouts` envia uma solicitação ao bridge para obter os layouts e retorna eles em formato de lista de objetos `Layout`. Cada objeto `Layout` contém informações sobre o layout, como seu nome e dados.