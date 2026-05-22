```rust
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SharedRootContext {
    deep_links: Vec<String>,
    app_configuration: Option<AppConfiguration>,
    data_sources: Vec<DataSourceFactory>,
    extension_loaders: Vec<IExtensionLoader>,
    native_app_menu: Option<NativeAppMenu>,
    native_window: Option<NativeWindow>,
    enable_launch_preference_screen: bool,
    enable_global_css: bool,
    appBar_leftInset: usize,
    extra_providers: Vec<React::Element>,
    custom_window_control_props: CustomWindowControlsProps,
    on_app_bar_double_click: Box<dyn Fn()>,
    AppBarComponent: Box<dyn Fn(AppBarProps) -> React::Element>,
}

impl SharedRootContext {
    pub fn new(
        deep_links: Vec<String>,
        app_configuration: Option<AppConfiguration>,
        data_sources: Vec<DataSourceFactory>,
        extension_loaders: Vec<IExtensionLoader>,
        native_app_menu: Option<NativeAppMenu>,
        native_window: Option<NativeWindow>,
        enable_launch_preference_screen: bool,
        enable_global_css: bool,
        appBar_leftInset: usize,
        extra_providers: Vec<React::Element>,
        custom_window_control_props: CustomWindowControlsProps,
        on_app_bar_double_click: Box<dyn Fn()>,
        AppBarComponent: Box<dyn Fn(AppBarProps) -> React::Element>,
    ) -> Self {
        Self {
            deep_links,
            app_configuration,
            data_sources,
            extension_loaders,
            native_app_menu,
            native_window,
            enable_launch_preference_screen,
            enable_global_css,
            appBar_leftInset,
            extra_providers,
            custom_window_control_props,
            on_app_bar_double_click,
            AppBarComponent,
        }
    }
}

pub fn use_shared_root_context() -> Rc<SharedRootContext> {
    Rc::new(SharedRootContext {
        // Initialize fields with default values or None as needed
        deep_links: Vec::new(),
        app_configuration: None,
        data_sources: Vec::new(),
        extension_loaders: Vec::new(),
        native_app_menu: None,
        native_window: None,
        enable_launch_preference_screen: false,
        enable_global_css: false,
        appBar_leftInset: 0,
        extra_providers: Vec::new(),
        custom_window_control_props: CustomWindowControlsProps {
            // Initialize fields with default values or None as needed
            // Example:
            // enabled: true,
            // icon_url: "https://example.com/icon.png",
        },
        on_app_bar_double_click: Box::new(|| {}),
        AppBarComponent: Box::new(|_| React::Element::Fragment),
    })
}

pub type SharedRootContextHandle = Rc<SharedRootContext>;
```

Note: This code is a simplified example and does not include the implementation of the `AppConfiguration`, `DataSourceFactory`, `NativeAppMenu`, `NativeWindow`, `CustomWindowControlsProps`, and `React` types. The actual implementations would depend on the specific requirements of your application.