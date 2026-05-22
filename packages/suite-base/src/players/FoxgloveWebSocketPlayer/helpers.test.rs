```rust
use std::str;

pub fn dataType_to_full_name(data_type: &str) -> String {
    if data_type.starts_with("text/text") {
        data_type.to_string()
    } else {
        format!("{}msg/{}", &data_type[..&data_type.rfind('/').unwrap_or(0)], &data_type[data_type.rfind('/').unwrap() + 1..])
    }
}

pub fn status_level_to_problem_severity(level: StatusLevel) -> String {
    match level {
        StatusLevel::INFO => "info",
        StatusLevel::WARNING => "warn",
        StatusLevel::ERROR => "error",
    }
}
```

Aqui está o código convertido para Rust:

1. A função `dataType_to_full_name` recebe uma string como entrada e verifica se é de tipo "text/text". Se for, retorna a string intacta. Caso contrário, cria um novo formato com "/msg/" adicionado antes do nome da mensagem.

2. A função `status_level_to_problem_severity` recebe um valor do tipo `StatusLevel` e retorna uma string que representa o problema correspondente (info, warn ou error).