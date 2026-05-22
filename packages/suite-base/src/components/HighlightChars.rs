```rust
use std::collections::HashSet;

fn highlight_chars(str: &str, indices: HashSet<usize>, offset: usize) -> String {
    str.chars()
        .enumerate()
        .map(|(i, c)| if indices.contains(&(i + offset)) { format!("<mark>{}</mark>", c) } else { c.to_string() })
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    let str = "The quick brown fox jumps over the lazy dog";
    let indices: HashSet<usize> = [4, 8, 12, 16].iter().cloned().collect();
    let offset = 3;

    println!("{}", highlight_chars(str, indices, offset));
}
```

O código Rust funciona da mesma forma que o anterior, usando um conjunto para armazenar as posições das correspondências e uma função `highlight_chars` para substituir os caracteres correspondentes pelos marcadores `<mark>`.