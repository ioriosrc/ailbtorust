```rust
pub const DATA_SOURCE_INFO: &str = include_str!("datasource_info.txt");
```

O código acima carrega o conteúdo do arquivo `datasource_info.txt` como um string em Rust. O comando `include_str!` é usado para fazer isso, e o nome da variável é definido no lugar de `'datasource_info'`.