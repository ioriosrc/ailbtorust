```rust
use crate::components::{DataSourceDialog::DataSourceOption, DataSourceDialog::SidebarItems, LightblickLogoText, Stack, TextMiddleTruncate};
use crate::context::{AnalyticsContext, PlayerSelectionContext, WorkspaceActionsContext};
use crate::services::{AppEvent};
use crate::theme::use_styles;
use react::prelude::*;

#[function_component]
pub fn Start() -> Html {
    let recent_sources = use_context(PlayerSelectionContext).recent_sources.clone();
    let classes = use_styles();

    let analytics = use_context(AnalyticsContext);
    let dialog_actions = use_context(WorkspaceActionsContext);

    let start_items = useMemo(() => {
        vec![
            DataSourceOption {
                key: "open-local-file",
                text: "Open Local Files",
                secondary_text: "Open files from your local machine",
                icon: Some(include!("data/icons/open_file.svg")),
                on_click: move || {
                    dialog_actions.dataSource.open("file");
                    analytics.log_event(AppEvent::DIALOG_SELECT_VIEW, Some("local"));
                },
            },
            DataSourceOption {
                key: "open-connection",
                text: "Open Connection",
                secondary_text: "Connect to a remote data source",
                icon: Some(include!("data/icons/open_connection.svg")),
                on_click: move || {
                    dialog_actions.dataSource.open("connection");
                    analytics.log_event(AppEvent::DIALOG_SELECT_VIEW, Some("live"));
                },
            },
        ]
    }, &[dialog_actions.dataSource]);

    html! {
        <Stack classes={classes.grid}>
            <header>
                <LightblickLogoText color="primary" />
            </header>
            <Stack gap={4}>
                <Stack gap={1}>
                    <Typography variant="h5" gutterBottom>
                        {use_context(|t| t.get("openDataSource"))}
                    </Typography>
                    {start_items.iter().map(|item| {
                        DataSourceOption {
                            key: item.key.clone(),
                            text: item.text.clone(),
                            secondary_text: item.secondary_text.clone(),
                            icon: Some(include!("data/icons/{}.svg".to_string())),
                            on_click: move || {
                                dialog_actions.dataSource.open(&item.key);
                                analytics.log_event(AppEvent::DIALOG_SELECT_VIEW, Some(&item.key));
                            },
                        }
                    }).collect::<Html>()}
                </Stack>
                {if recent_sources.len() > 0 {
                    <Stack gap={1}>
                        <Typography variant="h5" gutterBottom>
                            {use_context(|t| t.get("recentDataSources"))}
                        </Typography>
                        <List disable_padding>
                            {recent_sources.iter().take(5).map(|recent| {
                                let text = if recent.title.len() > 60 {
                                    TextMiddleTruncate::new(&recent.title[..60], None)
                                } else {
                                    TextMiddleTruncate::new(&recent.title, None)
                                };
                                ListItem disable_padding key={recent.id.clone()} id={recent.id.clone()}>
                                    <ListItemButton
                                        disable_gutters
                                        on_click: move || {
                                            dialog_actions.dataSource.open(&recent.key);
                                        }
                                    >
                                        {text}
                                    </ListItemButton>
                                </ListItem>
                            }).collect::<Html>()}
                        </List>
                    </Stack>
                }}
            </Stack>
        </Stack>
        <div classes={classes.spacer} />
        <Stack gap={4} classes={classes.sidebar}>
            <SidebarItems onSelect_view={dialog_actions.dataSource.open} />
        </Stack>
    }
}
```