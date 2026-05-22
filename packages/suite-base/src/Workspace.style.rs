```rust
use styled::{css, keyframes};
use styled::components::div;

fn main() {
    let classes = styles! {
        container: {
            width: "100%",
            height: "100%",
            display: "flex",
            flex_direction: "column",
            position: "relative",
            flex: "1 1 100%",
            outline: "none",
            overflow: "hidden",
        },
    };

    div!("Hello, World!", {
        classes,
    })
}
```

Este código cria uma classe CSS utilizando styled-components em Rust. A função `styles!` é utilizada para definir estilos de componentes React, similar ao que ocorre no TypeScript/React.