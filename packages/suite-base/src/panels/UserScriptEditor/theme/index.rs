```rust
use monaco::{editor, IStandaloneThemeData};

fn main() {
    let themes = vec![
        Theme {
            name: String::from("vs-studio-dark"),
            theme: IStandaloneThemeData::new(vs_studio_dark),
        },
        Theme {
            name: String::from("vs-studio-light"),
            theme: IStandaloneThemeData::new(vs_studio_light),
        },
    ];

    for theme in themes {
        println!("Theme name: {}", theme.name);
        println!("Theme data: {:?}", theme.theme);
    }
}
```

Este código Rust cria uma lista de temas do monaco e imprime o nome e os dados dos temas.