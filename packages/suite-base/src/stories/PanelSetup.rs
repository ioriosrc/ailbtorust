```rust
use std::collections::HashMap;

fn noop() {}

type Frame = HashMap<String, Vec<MessageEvent>>;

pub type Fixture = Option<HashMap<String, Topic>>;

struct UserScript {
    diagnostics: Vec<Diagnostic>,
    logs: Vec<UserScriptLog>,
}

struct GlobalVariables;

struct RosDatatypes;

struct SettingsTreeEditor;

struct UnconnectedPanelSetup {
    fixture: Option<HashMap<String, Topic>>,
    include_settings: bool,
    settings_width: Option<f32>,
}

impl Default for UnconnectedPanelSetup {
    fn default() -> Self {
        Self {
            fixture: None,
            include_settings: false,
            settings_width: None,
        }
    }
}

fn make_mock_panel_catalog(t: &str) -> HashMap<String, Box<dyn Panel>> {
    let all_panels = panels::builtin_modules().iter().map(|p| p.clone()).collect();

    let visible_panels = all_panels;

    HashMap::from_iter(visible_panels.into_iter().zip(vec![Box::new(MockPanel {})]))
}

struct MockPanel {}

impl Panel for MockPanel {
    fn type_name(&self) -> &'static str {
        "MockPanel"
    }
}

fn trigger_wheel(target: &Element, deltaX: i32) {
    let event = WheelEvent {
        delta_x,
        bubbles: true,
        cancelable: true,
    };
    target.dispatchEvent(event);
}

struct MosaicWrapper {
    children: Element,
}

impl Component for MosaicWrapper {
    type Props = ();

    fn render(&self) -> Html {
        html! {
            <DndProvider backend={HTML5Backend}>
                <Mosaic
                    className="mosaic-foxglove-theme" // prevent the default mosaic theme from being applied
                    initialValue="mock"
                    render_tile|(_id, path)| {
                        html! {
                            <MosaicWindow title="" path={path} render_preview={() => <div />}>
                                {self.children.clone()}
                            </MosaicWindow>
                        }
                    }
                />
            </DndProvider>
        }
    }
}

struct EmptyTree;

impl SettingsTreeEditor for EmptyTree {}

fn PanelWrapper {
    let settings = use_panel_state(|store| {
        let trees = store.settings_trees;
        if trees.len() > 1 {
            panic!("includeSettings requires there to be at most 1 panel, found {}", trees.len());
        }
        trees[0].clone()
    });

    html! {
        <>
            {settings.include_settings && (
                <div style={{ overflow: "auto", width: settings.settings_width.unwrap_or(200.0) }}>
                    <SettingsTreeEditor variant="panel" settings={settings} />
                </div>
            )}
            {self.children.clone()}
        </>
    }
}

struct DefaultFetchAsset;

impl Component for DefaultFetchAsset {
    type Props = ();

    fn render(&self) -> Html {
        html! {
            async move {
                let response = fetch(uri, options);
                Ok(response.body().await?)
            }
        }
    }
}

fn select_user_script_actions(store: &UserScriptStore) -> Vec<UserScriptAction>;

struct UnconnectedPanelSetupProps {
    fixture: Option<HashMap<String, Topic>>,
    include_settings: bool,
    settings_width: Option<f32>,
    on_layout_action: Option<Fn(PanelsActions)>,

    frame: HashMap<String, Vec<MessageEvent>>,
    topics: Vec<Topic>,
    datatypes: Option<RosDatatypes>,
    capabilities: Option<Vec<String>>,
    profile: String,
    active_data: Option<Omit<PlayerStateActiveData, "messages">>,
    progress: Progress,
    publish: Option<fn(PublishPayload)>,
    set_publishers: Option<fn(&str, Vec<AdvertiseOptions>)>,
    set_subscriptions: Option<ComponentProps<typeof MockMessagePipelineProvider>["setSubscriptions"]>,
    set_parameter: Option<fn(&str, &ParameterValue)>,
    fetch_asset: Option<ComponentProps<typeof MockMessagePipelineProvider>["fetchAsset"]>,
    call_service: Option<fn(&str, &dyn Fn(unknown) -> Result<unknown, anyhow::Error>)>,
    message_converters: Option<&[RegisterMessageConverterArgs<unknown>]>,
    panel_state: Option<HashMap<String, PanelState>>,
}

impl Default for UnconnectedPanelSetupProps {
    fn default() -> Self {
        Self {
            fixture: None,
            include_settings: false,
            settings_width: None,
            on_layout_action: None,

            frame: HashMap::new(),
            topics: Vec::new(),
            datatypes: Some(RosDatatypes::default()),
            capabilities: None,
            profile: String::from("mock"),
            active_data: None,
            progress: Progress::default(),
            publish: None,
            set_publishers: None,
            set_subscriptions: None,
            set_parameter: None,
            fetch_asset: None,
            call_service: None,
            message_converters: None,
            panel_state: Some(HashMap::new()),
        }
    }
}

fn make_mock_user_script_diagnostics() -> Vec<Diagnostic> {
    vec![]
}

fn make_mock_user_script_logs() -> Vec<UserScriptLog> {
    vec![]
}

async fn fetch_asset(uri: &str, options: RequestInit) -> Result<Vec<u8>, reqwest::Error> {
    Ok(reqwest::get(uri, options).await?.bytes().await?)
}

async fn call_service(service: &str, request: impl Fn(unknown) -> Result<unknown, anyhow::Error>) -> Result<unknown, anyhow::Error> {
    request(()?)
}

fn default_fetch_asset(_: &str, _: RequestInit) -> Result<Vec<u8>, reqwest::Error> {
    Ok(vec![])
}

struct PanelSetupProps {
    include_settings: bool,
    settings_width: Option<f32>,
    on_layout_action: Option<Fn(PanelsActions)>,

    fixture: Fixture,
}

impl Default for PanelSetupProps {
    fn default() -> Self {
        Self {
            include_settings: false,
            settings_width: None,
            on_layout_action: None,

            fixture: None,
        }
    }
}

fn main() {}
```