```rust
use tui::{style::Style, widgets::Table, widgets::Row};
use tui::layout::Constraint;
use std::collections::HashMap;

// Define the styles using the provided CSS classes and theme
fn create_styles(theme: &tui::style::Theme) -> HashMap<String, Style> {
    let mut styles = HashMap::new();

    styles.insert(String::from("resizeHandle"), Style::default().bg(Style::rgb(255, 0, 0)).fg(Style::rgb(255, 255, 255)).bold());
    styles.insert(String::from("table"), Style::default().bg(theme.background.paper));
    styles.insert(String::from("tableHeaderRow"), Style::default().bg(theme.background.paper).bold());
    styles.insert(String::from("htmlTableCell h1, h2, h3, h4, h5, h6"), Style::default()
        .fg(theme.typography.subtitle2.font_family)
        .font_size(theme.typography.subtitle2.fontSize)
        .line_height(theme.typography.subtitle2.line_height)
        .letter_spacing(theme.typography.subtitle2.letter_spacing)
        .bold());

    styles.insert(String::from("iconButton"), Style::default().bg(Style::rgb(0, 0, 0)).fg(Style::rgb(255, 255, 255)));

    styles
}

// Define the rows for the table using the provided data structure
fn create_table_rows(data: &[(&str, i32)]) -> Vec<Row> {
    let mut rows = Vec::new();

    for (item, count) in data {
        rows.push(Row::new().add_cell(item.to_string()).add_cell(count.to_string()));
    }

    rows
}

// Define the main function to create the table using the defined styles and rows
fn create_table(theme: &tui::style::Theme) -> Table {
    let mut table = Table::new()
        .header(vec![TableColumn::from("Item"), TableColumn::from("Count")])
        .widths(&[Constraint::flexible(1.0), Constraint::flexible(1.0)])
        .style(create_styles(theme))
        .block(TableBlock {
            title: None,
            style: Style::default(),
        });

    for row in create_table_rows(vec![("Apple", 5), ("Banana", 3)]) {
        table.add_row(row);
    }

    table
}
```