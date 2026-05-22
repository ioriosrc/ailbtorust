```rust
use crate::{components::LayoutBrowser, context::CurrentUserContext, hooks::useAppConfigurationValue};
use mui::{
    ButtonBase, IconButton, Link, Typography,
    ButtonBaseProps, IconButtonProps, LinkProps, TypographyProps, spacing,
};

type SignInPromptProps = LayoutBrowser::SignInPromptProps;

fn SignInPrompt(props: SignInPromptProps) -> impl 'static + JSX.Element {
    let { onDismiss } = props;
    let { signIn } = CurrentUserContext::use_context();
    let { classes } = useStyles();

    let action = if useAppConfigurationValue::<bool>(AppSetting::ENABLE_NEW_TOPNAV).unwrap_or_default() {
        Some(signIn)
    } else {
        Some(open_account_settings())
    };

    (
        ButtonBase::new(classes.root, {
            props: ButtonBaseProps {
                children: vec![
                    Typography::new(classes.title, {
                        props: TypographyProps {
                            align: "left",
                            variant: "body2",
                        },
                    }),
                    Link::new(
                        classes.link,
                        LinkProps {
                            color: "inherit",
                            underline: "always",
                        },
                        onDismiss.map_or("", |_| "Sign in"),
                    ),
                ],
            },
        })
        .on_click(move |_| action),
        if onDismiss.is_some() {
            IconButton::new(classes.close_icon, IconButtonProps {
                aria_label: "Dismiss".into(),
                size: "small",
                role: "button",
                children: Some(CloseIcon {}),
                on_click: move |_| {
                    event.stopPropagation();
                    onDismiss.unwrap_or(()).execute();
                },
            })
        } else {
            None
        }
    )
}
```