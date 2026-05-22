```rust
use path::PathBuf;

#[derive(Debug)]
pub struct ConfigParams {
    pub output_path: PathBuf,
    pub context_path: PathBuf,
    pub entrypoint: PathBuf,
    pub prod_source_map: &'static str,
    pub version: String,
}

// foxglove-depcheck-used: webpack-dev-server
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = ConfigParams {
        output_path: PathBuf::from(".webpack"),
        context_path: PathBuf::from("src"),
        entrypoint: PathBuf::from("./entrypoint.tsx"),
        prod_source_map: "source-map",
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(vec![dev_server_config(params), main_config(params)])
}
```

**Explicação**:
1. **Structura de dados**: A estrutura `ConfigParams` é definida para encapsular as configurações necessárias para o Webpack. Ela inclui campos como `output_path`, `context_path`, `entrypoint`, `prod_source_map`, e `version`.
2. **Valor da versão**: O valor de `version` é obtido através do env var `CARGO_PKG_VERSION` para garantir que a versão esteja disponível quando o código é compilado.
3. **Função principal**: A função `main` retorna um resultado de erro, pois ela pode falhar (por exemplo, se os diretórios ou arquivos não existirem). Se houver problemas, ele retorna uma mensagem de erro usando `Box<dyn std::error::Error>`.
4. **Configurações do Webpack**: As configurações são criadas e passadas à função `dev_server_config` e `main_config`, que devem ser implementadas separadamente para configurar a parte específica do Webpack para o desenvolvimento e produção.