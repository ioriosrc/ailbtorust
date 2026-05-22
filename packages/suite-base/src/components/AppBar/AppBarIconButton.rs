```rust
use derive_mui_core::IconButton;
use mui_system::{SystemProps, ButtonBaseProps};
use mui_base::button::ButtonBase;

pub struct AppBarIconButton {
    props: AppBarIconButtonProps,
}

impl AppBarIconButton {
    pub fn new(props: AppBarIconButtonProps) -> Self {
        Self { props }
    }

    fn render(&self) -> String {
        let mut classes = "MuiIconButton-root".to_string();
        if let Some(custom_class_name) = &self.props.className {
            classes.push_str(" ").push_str(custom_class_name);
        }
        let color = match &self.props.color {
            "inherit" => "primary",
            _ => color,
        };

        format!(
            r#"<Tooltip disableInteractive arrow title='{}' enterDelay={200} class="{}">{}</Tooltip>"#,
            self.props.title.unwrap_or(""),
            classes,
            generate_icon_button(color)
        )
    }
}

fn generate_icon_button(color: &str) -> String {
    let icon_color = color.to_lowercase();
    if icon_color.eq_ignore_ascii_case("primary") || icon_color.eq_ignore_ascii_case("success") {
        return format!(
            r#"<svg viewBox="0 0 24 24" fill="{}"><path d="M12 9.75V3L8.62 8.62L12 9.75Z"/></svg>"#,
            icon_color
        );
    } else if color.eq_ignore_ascii_case("error") {
        return format!(
            r#"<svg viewBox="0 0 24 24" fill="{}"><path d="M12 9.75V3L8.62 8.62L12 9.75Z"/></svg>"#,
            icon_color
        );
    } else {
        return format!(
            r#"<svg viewBox="0 0 24 24" fill="{}"><path d="M12 9.75V3L8.62 8.62L12 9.75Z"/></svg>"#,
            icon_color
        );
    }
}

fn main() {
    // Example usage:
    let appBarIconButton = AppBarIconButton::new({
        title: Some("Menu"),
        color: "primary",
        className: Some("custom-class"),
        children: String::from("Menu Icon"),
    });
    println!("{}", appBarIconButton.render());
}
```