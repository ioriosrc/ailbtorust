```rust
use serde_json::json;

pub fn process(source_text: &str) -> String {
    format!("module.exports = {};", json!(source_text))
}
```

Explicação:
1. Importamos a biblioteca `serde_json` para manipular JSON.
2. Definimos a função `process` que recebe uma string como argumento e retorna uma nova string.
3. Usamos a função `json!` do `serde_json` para convertir a string original em um objeto JSON.
4. Concatenamos o prefixo `"module.exports = "` com o objeto JSON resultante.
5. Retornamos a string resultante.

Essa implementação é funcional porque ela usa Rust's tipo de dados padrão e funções prontas do语言, sem necessitar de imports adicionais ou manipulação explícita de strings.