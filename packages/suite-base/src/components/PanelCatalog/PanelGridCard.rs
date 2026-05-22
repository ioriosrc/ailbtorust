```rust
use react::prelude::*;

use crate::PanelInfo;

#[derive(PartialEq, Eq)]
enum PanelState {
  Normal,
  Highlighted,
}

#[function_component]
fn PanelGridCard(props: Props) -> HtmlElement {
  let props = props.clone();
  let search_query = props.search_query;
  let onClick = props.onClick;
  let panel = props.panel;

  let target_string = if panel.extension_namespace.is_some() {
    format!("{} [{}] ", panel.title, panel.extension_namespace.unwrap())
  } else {
    panel.title.to_string()
  };

  let onClick_with_stop_propagation = use_callback(
    move |event: MouseEvent| {
      event.stopPropagation();
      onclick();
    },
    [onClick],
  );

  html! {
    <Card>
      <CardActionArea onClick={onClick_with_stop_propagation}>
        <Stack full_height>
          {panel.thumbnail.is_some() {
            html! {
              <CardMedia component="img" image=props.panel.thumbnail.clone().unwrap_or_default() alt=props.panel.title.clone() />
            }
          } else {
            html! {
              <div className={classes.image_placeholder} />
            }
          }}
          <CardContent>
            <Typography variant="subtitle2" gutter_bottom>
              <span data-testid={`panel-grid-card ${panel.title}`}>
                <TextHighlight target_str=target_string.to_string() searchText=search_query.clone() />
              </span>
            </Typography>
            <Typography variant="body2" color="text.secondary">
              <TextHighlight target_str=props.panel.description.unwrap_or_default().to_string() searchText=search_query.clone() />
            </Typography>
          </CardContent>
        </Stack>
      </CardActionArea>
    </Card>
  }
}

#[derive(PartialEq, Eq)]
enum PanelState {
  Normal,
  Highlighted,
}
```