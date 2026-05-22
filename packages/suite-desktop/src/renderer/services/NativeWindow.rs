```rust
use std::sync::Arc;

pub struct NativeWindow {
    bridge: Arc<dyn Desktop>,
}

impl NativeWindow {
    pub fn new(bridge: Arc<dyn Desktop>) -> Self {
        NativeWindow { bridge }
    }

    pub async fn set_represented_filename(&self, path: Option<&str>) {
        self.bridge.set_represented_filename(path);
    }

    pub fn on<E>(&self, name: E, listener: fn()) where E: NativeWindowEvent {
        self.bridge.add_ipc_event_listener(name, listener);
    }

    pub fn handle_title_bar_double_click(&self) {
        self.bridge.handle_title_bar_double_click();
    }

    pub fn is_maximized(&self) -> bool {
        self.bridge.is_maximized()
    }

    pub fn minimize(&self) {
        self.bridge.minimize_window();
    }

    pub fn maximize(&self) {
        self.bridge.maximize_window();
    }

    pub fn unmaximize(&self) {
        self.bridge.unmaximize_window();
    }

    pub fn close(&self) {
        self.bridge.close_window();
    }

    pub fn reload(&self) {
        self.bridge.reload_window();
    }
}
```
O código Rust foi adaptado para ser funcional e modular. Utilizou o padrão de design `struct` para representar a classe `NativeWindow`, onde o campo `bridge` é uma referência de ponteiro arredondada (`Arc`). O método `new` cria uma nova instância da classe, passando a referencia ao bridge. O método `set_represented_filename` envia uma solicitação para o bridge para definir o nome do arquivo representado pelo navegador. O método `on` registra um listener para eventos específicos do bridge. As demais métodos correspondem aos comportamentos padrões de maximização, minimização e fechamento da janela.