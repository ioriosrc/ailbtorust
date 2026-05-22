```rust
use std::sync::{Arc, Mutex};

pub type InstallingProgress = (u64, u64, bool);

pub type InstallingExtensionsState = Arc<Mutex<InstallingExtensionsData>>;

#[derive(Clone)]
struct InstallingExtensionsData {
    installed: u64,
    total: u64,
    in_progress: bool,
}

impl Default for InstallingExtensionsData {
    fn default() -> Self {
        Self {
            installed: 0,
            total: 0,
            in_progress: false,
        }
    }
}

pub struct InstallingExtensionsStore {
    state: Arc<Mutex<InstallingExtensionsData>>,
}

impl InstallingExtensionsStore {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(InstallingExtensionsData::default())),
        }
    }

    pub fn set_installing_progress(&self, progress_fn: impl FnMut(&mut InstallingExtensionsData) -> InstallingExtensionsData) {
        let mut state = self.state.lock().unwrap();
        *state = progress_fn(state);
    }

    pub fn start_installing_progress(&self, extensions_to_be_installed: u64) {
        let mut state = self.state.lock().unwrap();
        state.total = extensions_to_be_installed;
        state.installed = 0;
        state.in_progress = true;
    }

    pub fn reset_installing_progress(&self) {
        let mut state = self.state.lock().unwrap();
        state.total = 0;
        state.installed = 0;
        state.in_progress = false;
    }
}
```

### Explicação

1. **Definição de Tipos**:
   - `InstallingProgress`: Uma tupla que representa o estado atual do progresso de instalação.
     ```rust
     type InstallingProgress = (u64, u64, bool);
     ```
   - `InstallingExtensionsState`: Um gerenciador de estado para armazenar e manipular o estado da instalação de extensões.
     ```rust
     pub struct InstallingExtensionsState {
         state: Arc<Mutex<InstallingExtensionsData>>,
     }
     ```
   - `InstallingExtensionsData`: Uma estrutura que representa o estado atual do progresso.
     ```rust
     #[derive(Clone)]
     struct InstallingExtensionsData {
         installed: u64,
         total: u64,
         in_progress: bool,
     }

     impl Default for InstallingExtensionsData {
         fn default() -> Self {
             Self {
                 installed: 0,
                 total: 0,
                 in_progress: false,
             }
         }
     }
     ```

2. **Implementação do Gerenciador de Estado**:
   - `InstallingExtensionsStore`: Uma implementação da estrutura que gerencia o estado do progresso.
     ```rust
     impl InstallingExtensionsStore {
         pub fn new() -> Self {
             Self {
                 state: Arc::new(Mutex::new(InstallingExtensionsData::default())),
             }
         }

         pub fn set_installing_progress(&self, progress_fn: impl FnMut(&mut InstallingExtensionsData) -> InstallingExtensionsData) {
             let mut state = self.state.lock().unwrap();
             *state = progress_fn(state);
         }

         pub fn start_installing_progress(&self, extensions_to_be_installed: u64) {
             let mut state = self.state.lock().unwrap();
             state.total = extensions_to_be_installed;
             state.installed = 0;
             state.in_progress = true;
         }

         pub fn reset_installing_progress(&self) {
             let mut state = self.state.lock().unwrap();
             state.total = 0;
             state.installed = 0;
             state.in_progress = false;
         }
     }
     ```

3. **Funções de Modificação**:
   - `set_installing_progress`: Função para modificar o estado do progresso.
   - `start_installing_progress`: Função para iniciar a instalação e definir o total de extensões.
   - `reset_installing_progress`: Função para resetar o estado do progresso.

4. **Locking**:
   - O estado é protegido pelo mecanismo de bloqueio (`Arc<Mutex<InstallingExtensionsData>>`) para garantir thread-safety.

### Usabilidade

Para usar essa implementação, você pode criar uma instância do gerenciador e chamar as funções de modificação conforme necessário. Por exemplo:

```rust
fn main() {
    let store = InstallingExtensionsStore::new();

    // Função para atualizar o progresso da instalação
    let update_progress = |progress: &mut InstallingExtensionsData| {
        if progress.total > 0 {
            progress.installed += 1;
        }
        *progress
    };

    // Start the installation with 10 extensions
    store.set_installing_progress(update_progress);

    // Simular o carregamento das extensões (por exemplo, usando uma delay)
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Reset the progress after installation
    store.reset_installing_progress();
}
```

Este código demonstra como implementar um gerenciador de estado para monitorar e manipular o progresso da instalação de extensões em Rust, usando `zustand` para armazenar e manter o estado.