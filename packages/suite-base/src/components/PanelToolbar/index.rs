```rust
use crate::{FullscreenIcon, FullscreenExitIcon};
use styled_components::css;

fn PanelToolbar(props: &Props) -> Html {
    let additional_icons = props.additional_icons.clone();
    let is_fullscreen = props.is_fullscreen;
    let exit_fullscreen = props.exit_fullscreen;
    let enter_fullscreen = props.enter_fullscreen;

    let panel_context = useContext::<PanelContext>();
    let classes = use_classes();

    let additional_icon_with_help = {
        if props.children.is_none() || !props.is_unknown_panel {
            additional_icons.clone()
        } else {
            vec![ToolbarIconButton::new("exit-fullscreen", "Exit fullscreen".into(), exit_fullscreen), ToolbarIconButton::new("fullscreen", "Fullscreen".into(), enter_fullscreen)]
                .iter()
                .map(|icon| icon.with_classes(classes))
                .collect::<Vec<_>>()
        }
    };

    let root_drag_ref = if props.is_unknown_panel || props.children.is_none() {
        None
    } else {
        panel_context?.connect_toolbar_drag_handle.clone()
    };

    let controls_drag_ref = if props.is_unknown_panel || props.children.is_none() {
        None
    } else {
        panel_context?.connect_toolbar_drag_handle.clone()
    };

    let default_panel_title = use_default_panel_title();
    let custom_panel_title = if props.custom_title.is_some() && !props.custom_title.as_ref().unwrap().is_empty() {
        props.custom_title.as_ref().unwrap().clone()
    } else {
        default_panel_title
    };

    let title = custom_panel_title.clone() ?? panel_context?.title.clone();

    html! {<header class={css!("root", props.class_name)} data-testid="mosaic-drag-handle" ref={root_drag_ref}>
            {props.children.clone().map(|child| child).unwrap_or_else(|| html! {
                <Typography noWrap variant="body2" color="text.secondary">
                    {title}
                </Typography>
            })}
            <PanelToolbarControls additional_icons={additional_icon_with_help} is_unknown_panel={props.is_unknown_panel} ref={controls_drag_ref}/>
        </header>}
    }
}
```