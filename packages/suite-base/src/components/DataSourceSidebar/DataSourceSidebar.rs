```rust
use styled::{css, Theme};
use mui::material::{
  Tabs, Tab, Stack, IconButton, CircularProgress, Divider,
};

use crate::pages::DataSourceInfoView;
use crate::pages::TopicList;
use crate::pages::EventsList;
use crate::pages::AlertsList;

type Props = {
  disableToolbar: bool;
};

fn useStyles(theme: &Theme) -> css! {
  display: "flex",
  flex: "auto",
  overflow: "auto",
  padding: theme.spacing(2, 1),
  text_transform: "uppercase",
  font_weight: "bold",
  letter_spacing: "0.1rem",
};

fn TabContent(props: &Props) -> impl 'static {
  let { disableToolbar } = props;

  Stack {
    flex: "auto",
    padding: disable_toolbar.then_some(theme.spacing(2, 1)),
    background_color: theme.palette.background.paper,
    border_radius: theme.border_radius.md,
    box_shadow: "0 4px 8px rgba(0, 0, 0, 0.1)",
  }
}

fn AlertCount(props: &Props) -> impl 'static {
  let { disableToolbar } = props;

  Stack::flex_column()
    .padding_x(theme.spacing(2, 1))
    .padding_y(theme.spacing(0.5))
    .border_radius(theme.border_radius.md)
    .background_color(theme.palette.error.main)
    .text_transform("uppercase")
    .font_weight("bold")
    .letter_spacing("0.1rem")
}

fn DataSourceSidebar(props: Props) -> impl 'static {
  let { disableToolbar } = props;

  SidebarContent::default()
    .disable_padding()
    .disable_toolbar(disableToolbar)
    .overflow("auto")
    .title("DataSource", t!("dataSource"))
    .trailing_items(vec![
      isLoading().map(|is_loading| {
        if is_loading {
          Stack::flex_column()
            .align_items("center")
            .justify_content("center")
            .padding_x(theme.spacing(2, 1))
            .box_shadow("0 4px 8px rgba(0, 0, 0, 0.1)")
            .children(vec![CircularProgress::default().size(18).variant("indeterminate")])
        } else {
          None
        }
      }),
      IconButton::new()
        .color("primary")
        .title(t!("New connection"))
        .on_click(|| dialog_actions.dataSource.open("start")),
    ])
    .children(vec![
      !disable_toolbar.then_some(TabContent::default()),
      if show_events_tab() && selected_event_id.is_some() {
        Tab::new().value("events").label("Events")
      },
      Tab::new()
        .value("alerts")
        .label(format!(
          "Alerts ({})",
          alert_count.map_or("", |alert_count| alert_count.to_string())
        )),
    ])
    .children(vec![
      TopicsList::default(),
      EventsList::default().if(show_events_tab() && selected_event_id.is_some()),
      AlertsList::default(),
    ])
    .children(vec![WssErrorModal::new(player_alerts)]);
}
```