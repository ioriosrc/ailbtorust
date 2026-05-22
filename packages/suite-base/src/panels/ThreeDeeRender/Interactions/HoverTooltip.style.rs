```rust
use tui::style::{Color, Style};
use tui::widgets::{Table, TableState};

fn create_table_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::LightBlue)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut table = Table::new()
        .rows(vec![
            TableRow::from(["Key1".to_string(), "Value1".to_string()]),
            TableRow::from(["Key2".to_string(), "Value2".to_string()]),
        ])
        .column_header(TableColumn::new("Key").style(create_table_style()))
        .column_header(TableColumn::new("Value").style(create_table_style()));

    table.set_state(TableState::default());

    // Add your code to render and handle the table
    Ok(())
}
```