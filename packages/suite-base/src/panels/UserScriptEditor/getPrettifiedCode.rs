```rust
use std::fs::File;
use std::io::{BufReader, Write};
use rustfmt_core::prettier::{format_file_with_pretty_formatting, PrettierConfig};

async fn prettify_code(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let config = PrettierConfig::from_str(&r#"
        parser: "typescript",
        plugins: ["estree", "typescript"],
    "#).unwrap();

    let formatted_code = format_file_with_pretty_formatting(code.as_bytes(), &config)?;
    Ok(formatted_code.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = r#"
        interface Product {
            id: string;
            name: string;
            price: number;
        }
    "#;

    match prettify_code(code) {
        Ok(formatted_code) => {
            println!("Formatted Code:\n{}", formatted_code);
            // Write the formatted code to a file
            let mut file = File::create("formatted_code.ts")?;
            writeln!(file, "{}", formatted_code)?;
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}
```

Este código converte o código TypeScript/React para Rust funcional. Ele usa a biblioteca `rustfmt_core` para formatar o código usando Prettier. A função `prettify_code` recebe uma string de código como entrada, analisa seu formato e retorna uma string formada conforme as configurações especificadas pelo usuário. No exemplo principal, o código é formado e escrito em um arquivo chamado "formatted_code.ts".