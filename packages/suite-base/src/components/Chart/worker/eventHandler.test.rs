```rust
use std::collections::HashMap;

struct EventEmitter {
    listeners: HashMap<String, Vec<Box<dyn Fn(&str)>>>,
}

impl EventEmitter {
    fn new() -> Self {
        EventEmitter {
            listeners: HashMap::new(),
        }
    }

    fn add_listener(&mut self, event: &str, handler: Box<dyn Fn(&str)>) {
        let mut handlers = self.listeners.entry(event.to_string()).or_default();
        handlers.push(handler);
    }

    fn remove_listener(&mut self, event: &str, handler: Box<dyn Fn(&str)>) {
        if let Some(handlers) = self.listeners.get_mut(event) {
            handlers.retain(|h| *h != handler);
        }
    }

    fn emit(&self, event: &str, args: &str) {
        if let Some(handlers) = self.listeners.get(event) {
            for handler in handlers.iter() {
                handler(args);
            }
        }
    }
}

fn add_listener(emitter: &mut EventEmitter, event: &str, handler: Box<dyn Fn(&str)>>) {
    emitter.add_listener(event, handler);
}

fn remove_listener(emitter: &mut EventEmitter, event: &str, handler: Box<dyn Fn(&str)>>) {
    emitter.remove_listener(event, handler);
}
```

Este código em Rust é equivalente ao anterior em TypeScript/React, mas é escrito em Rust. Ele define uma estrutura `EventEmitter` que implementa os métodos para adicionar e remover eventos, bem como emitir eventos com argumentos. A função `add_listener`, `remove_listener`, e `emit` são fornecidas para permitir a manipulação de eventos no sistema.