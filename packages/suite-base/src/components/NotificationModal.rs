```rust
use mui::icons::material::Close;
use mui::dialog::{Dialog, DialogTitle, IconButton};
use mui::typography::{Typography, TypographyProps};
use mui::theme::create_use_theme;
use mui::style_system::{use_tss, makeStyles, CSSProperties};

use std::error::Error;

#[derive(Debug)]
pub struct NotificationMessage {
    details: Option<String>,
    message: String,
    severity: String,
    subText: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let theme = create_use_theme();
    let classes = useStyles(theme);

    let notification_message = NotificationMessage {
        details: Some("Error occurred".to_string()),
        message: "An error has occurred".to_string(),
        severity: "error".to_string(),
        subText: None,
    };

    let dialog = Dialog::new()
        .classes(classes.paper)
        .full_width(true)
        .open(notification_message.message == "An error has occurred")
        .on_close(|| println!("Dialog closed"))
        .render(|dialog| {
            dialog.render_header(|header| {
                header.title TypographyProps::new().style({ color: classes.severity }).text(&notification_message.message);
            });

            if notification_message.subText.is_some() {
                dialog.render_body(|body| {
                    body.text(&notification_message.subText.as_ref().unwrap());
                });
            }

            dialog.render_footer(|footer| {
                footer.render_button(|button| {
                    button.icon IconButtonProps::new().aria_label("close").icon(Close).onClick(|| println!("Dialog closed"));
                });
            });

            dialog.render_body(|body| {
                body.text(&notification_message.details.as_ref().unwrap());
            });
        })
        .render();

    Ok(())
}
```