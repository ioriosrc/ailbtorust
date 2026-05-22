```rust
use std::error::Error;

pub struct ErrorInfo<'a> {
    pub error: Box<dyn 'a + Error>,
    pub info: &'static str,
}

pub type GenericPanelProps<Config> = {
    child_id: Option<String>,
    override_config: Option<Config>,
    tab_id: Option<String>,
};

pub interface PanelStatics<Config> {
    panel_type: String;
    default_config: Config;
}

pub struct SetVisibleLogs {
    show: bool,
}

pub type PanelContextType<T> = {
    show_logs: Option<bool>;
    set_show_logs: fn(&mut Self, &SetVisibleLogs) -> (),
    log_error: fn(&mut Self, &str, Option<&Error>) -> (),
    log_count: usize,
    type_: String;
    id: String;
    title: String;
    tab_id: Option<String>;

    config: PanelConfig;
    save_config: SaveConfig<T>;

    update_panel_configs: fn(&mut Self, &str, fn(&T) -> T) -> (),
    open_sibling_panel: fn(),
    replace_panel: fn(&str, &Record<Str, unknown>),
    enter_fullscreen: fn() -> (),
    exit_fullscreen: fn() -> (),
    is_fullscreen: bool;

    /** Used to adjust z-index settings on parent panels when children are fullscreen */
    // eslint-disable-next-line @lichtblick/no-boolean-parameters
    set_has_fullscreen_descendant: fn(bool) -> (),
    connect_toolbar_drag_handle: Option<fn(&Element | ReactNull)>,
    setMessagePathDropConfig: fn(&mut Self, &Option<MessagePathDropConfig>) -> (),
};

pub type PanelErrorBoundaryProps = {
    show_error_details: bool,
    hide_error_source_locations: bool,
    on_reset_panel: fn(),
    on_remove_panel: fn(),
    on_log_error: fn(&str, Option<&Error>) -> (),
};

pub type PanelErrorBoundaryState = {
    current_error: Option<{ error: Box<dyn Error>, info: &'static str }>,
};

pub type PanelLog = { timestamp: String; message: String; error: Option<Error> };

pub type PanelLogsProps = {
    logs: Vec<PanelLog>;
    onClose: fn(),
    on_clear: fn(),
    initial_height: Option<usize>,
    on_height_change: fn(usize) -> (),
};

#[derive(Clone, Debug)]
pub enum OperationStatus {
    IDLE,
    INSTALLING,
    UNINSTALLING,
}

/** ExtensionDetail */
pub type ExtensionDetailsProps = {
    installed: bool;
    extension: Immutable<ExtensionMarketplaceDetail>;
    onClose: fn(),
};
```