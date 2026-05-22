```rust
use std::collections::HashMap;

fn main() {
    // Criação de um HashMap para armazenar os dados do TypeScript/React
    let mut data = HashMap::new();

    // Adição de dados ao HashMap
    data.insert("extends", "@lichtblick/tsconfig/base");
    data.insert("include", vec!["./**/*.ts"]);
    data.insert("compilerOptions", serde_json::to_string(&serde_json::Value::Object({
        "noEmit": true,
        "rootDir": ".",
        "lib": ["es2022"],
        "module": "NodeNext",
        "moduleResolution": "NodeNext"
    })).unwrap());

    // Exibição dos dados do HashMap
    println!("{:?}", data);
}
```