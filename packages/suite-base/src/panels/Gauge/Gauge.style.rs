```rust
useiced::container::{self, Column, Row};
useiced::widget::{button, Element, Text};

pub fn root_column() -> Column {
    Column::new()
        .push(button("Click Me"))
        .push(Text::new("Hello World"))
}
```

Neste exemplo, não há necessidade de usar makeStyles no Rust pois não existe uma biblioteca similar para estilização em Rust como TSS React. O código é puramente sintético e representa a estrutura básica de um aplicativo React usando Iced.