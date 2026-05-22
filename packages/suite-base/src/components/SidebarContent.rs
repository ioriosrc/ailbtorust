```rust
use std::any::Any;

use react_native::{
    components::Stack,
    style::{FlexStyle, Overflow},
};

type SidebarContentProps = {
    title: Option<String>,
    disable_padding: bool,
    disable_toolbar: bool,

    /** Buttons/items to display on the leading (left) side of the header */
    leading_items: Option<Vec<ReactNode>>,

    /** Overflow style of root element
     * @default: "auto"
     */
    overflow: Overflow,

    /** Buttons/items to display on the trailing (right) side of the header */
    trailing_items: Option<Vec<ReactNode>>,
};

fn SidebarContent(props: SidebarContentProps) -> ReactElement {
    let leading_items = props.leading_items.unwrap_or_default();
    let trailing_items = props.trailing_items.unwrap_or_default();

    let toolbar_style = if !props.disable_toolbar {
        flex: 0,
        align_self: "center",
        margin_left: -1,
        gap: 0.5,
    } else {
        FlexStyle::default()
    };

    let leading_items_style = flex: 1, align_self: "auto", overflow: props.overflow;

    let trailing_items_style = Stack {
        direction: StackDirection::Row,
        justify_content: JustifyContent::Center,
    }
    .props(FlexStyle {
        margin_left: -1,
        gap: 0.5,
    });

    let children_style = flex: 1;

    return (
        <Stack overflow={props.overflow} full_height flex="auto" gap={1}>
            {if !props.disable_toolbar {
                Box::new(
                    Column {
                        children: leading_items
                            .iter()
                            .map(|item| match item {
                                ReactNode::Component(com) => com,
                                _ => panic!("Only components are allowed in `leading_items`"),
                            })
                            .collect::<Vec<Box<dyn Any>>>(),
                    }
                    .style(leading_items_style),
                )
            } else {
                Box::new(Column {})
            }}
            <Box style(children_style)}>
                {props.title.map(|title| Text::new(title).style(TypographyProps {
                    variant: TypographyVariant::H4,
                    fontWeight: TypographyWeight::Bold,
                    flex: 1,
                }))}
            </Box>
            {if !props.disable_toolbar {
                Box::new(
                    Column {
                        children: trailing_items
                            .iter()
                            .map(|item| match item {
                                ReactNode::Component(com) => com,
                                _ => panic!("Only components are allowed in `trailing_items`"),
                            })
                            .collect::<Vec<Box<dyn Any>>>(),
                    }
                    .style(trailing_items_style),
                )
            } else {
                Box::new(Column {})
            }}
        </Stack>
    );
}
```