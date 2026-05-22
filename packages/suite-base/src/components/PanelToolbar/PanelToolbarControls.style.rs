```rust
use crate::materialui as mui;

pub fn useStyles() -> mui::styles::Theme } {
    mui::styles::makeStyles({
        name: "LogsBadge",
        styles: |state, props| {
            let theme = state.theme;

            {
                let dot_style = mui::style::Keyframe::new()
                    .duration("1s")
                    .animation_name(mui::anim::AnimName::Default)
                    .build();

                mui::style::Keyframes::chain(vec![dot_style])
                    .build()
                    .to(&[
                        ("&.MuiBadge-dot", |state| {
                            let size = theme.spacing(2);
                            vec![
                                (mui::style::Property::Width => size),
                                (mui::style::Property::Height => size),
                                (mui::style::Property::BorderRadius => "50%"),
                            ]
                        }),
                    ])
            }
        },
    })
}
```

Note: This Rust code is a simplified example and might not compile directly with the `tss-react/mui` crate. You would need to use a more complete setup with TypeScript/React to achieve this functionality, which involves creating a custom CSS-in-JS system or using an existing framework that supports such styling.