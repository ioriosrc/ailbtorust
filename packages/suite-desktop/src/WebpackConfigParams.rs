```rust
use std::path::{Path, PathBuf};

pub type WebpackConfigParams = (
    PathBuf,
    PathBuf,
    PathBuf,
    PathBuf,
    PathBuf,
    String,
    Option<String>,
);
```

Este código corresponde ao tipo `WebpackConfigParams` no TypeScript/React. Ele define uma tupla onde cada elemento representa um diretorio ou uma string de opção necessária para configurar o webpack.