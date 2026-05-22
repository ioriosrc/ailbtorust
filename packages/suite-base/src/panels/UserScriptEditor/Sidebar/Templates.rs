```rust
useiced::widget::{Column, List, ListItem, ListItemButton, ListItemText};
useiced::{
    Column as IcedColumn,
    Element, Window as IcedWindow,
    Text,
    button,
};

pub fn templates() -> Element<'static> {
    let mut list = List::new(|_| ());
    list.push(
        ListItem::new()
            .disable_padding()
            .on_click(move |_| {
                // Implement the logic to create a new script
                ()
            })
            .child(Column::with_children(vec![
                Text::new("Template 1").into(),
                Text::new("Description for template 1").into(),
            ])
            .into()),
    );
    list.push(
        ListItem::new()
            .disable_padding()
            .on_click(move |_| {
                // Implement the logic to create a new script
                ()
            })
            .child(Column::with_children(vec![
                Text::new("Template 2").into(),
                Text::new("Description for template 2").into(),
            ])
            .into()),
    );

    IcedColumn::new(list).into()
}
```