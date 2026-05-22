```rust
use crate::{shared_root_context, use_shared_root_context};
use leptos::{
    html,
    prelude::*,
    // Add other necessary imports here
};

#[component]
pub fn StudioApp() -> impl IntoView {
    let (
        data_sources,
        extension_loaders,
        native_app_menu,
        native_window,
        deep_links,
        enable_launch_preference_screen,
        extra_providers,
        appBar_left inset,
        custom_window_control_props,
        on_app_bar_double_click,
        AppBarComponent,
    ) = use_shared_root_context();

    let providers: Vec<impl IntoView> = vec![
        // Add other necessary imports here
        TimelineInteractionStateProvider {},
        ExtensionMarketplaceProvider {},
        ExtensionCatalogProvider {
            loaders: extension_loaders.clone(),
        },
        UserScriptStateProvider {},
        PlayerManager {
            player_sources: data_sources.clone(),
        },
        EventsProvider {},
    ];

    if let Some(extra_providers) = extra_providers.as_ref() {
        providers.extend(extra_providers.iter().map(|p| p.into_view()));
    }

    if native_app_menu.is_some() {
        providers.push(NativeAppMenuContext::new(native_app_menu.clone()).into_view());
    }

    if native_window.is_some() {
        providers.push(NativeWindowContext::new(native_window.clone()).into_view());
    }

    // The toast and logs provider comes first so they are available to all downstream providers
    providers.insert(0, StudioToastProvider.into_view());
    providers.insert(0, StudioLogsSettingsProvider.into_view());

    // Alerts provider also must come before other, dependent contexts.
    providers.insert(0, AlertsContextProvider.into_view());
    providers.insert(0, CurrentLayoutProvider.into_view());
    providers.insert(0, UserProfileLocalStorageProvider::new().into_view());
    providers.insert(0, LayoutManagerProvider {}.into_view());

    let layout_storage = IdbLayoutStorage::new();
    providers.push(LayoutStorageContext::new(layout_storage.clone()).into_view());

    if let Some(remote_layout_storage) = remote_layout_storage {
        providers.push(RemoteLayoutStorageContext::new(remote_layout_storage.clone()).into_view());
    }

    use_effect(move || {
        document.addEventListener("contextmenu", context_menu_handler);
        move || {
            document.removeEventListener("contextmenu", context_menu_handler);
        }
    });

    html! {
        <MaybeLaunchPreference>
            <MultiProvider providers={providers}>
                <DocumentTitleAdapter />
                <SendNotificationToastAdapter />
                <DndProvider backend=HTML5Backend::new()>
                    <Suspense fallback={<></>}>
                        <PanelCatalogProvider>
                            <Workspace
                                deep_links={deep_links.clone()}
                                appBar_left_inset={app_bar_left_inset}
                                on_app_bar_double_click={on_app_bar_double_click}
                                show_custom_window_controls={
                                    custom_window_control_props.as_ref()
                                        .map(|props| props.show_custom_window_controls)
                                        .unwrap_or_default()
                                }
                                is_maximized={
                                    custom_window_control_props.as_ref()
                                        .map(|props| props.is_maximized)
                                        .unwrap_or_default()
                                }
                                initial_zoom_factor={
                                    custom_window_control_props.as_ref()
                                        .map(|props| props.initial_zoom_factor)
                                        .unwrap_or_default()
                                }
                                on_minimize_window={
                                    custom_window_control_props
                                        .as_ref()
                                        .map(|props| props.on_minimize_window)
                                        .unwrap_or_default()
                                }
                                on_maximize_window={
                                    custom_window_control_props
                                        .as_ref()
                                        .map(|props| props.on_maximize_window)
                                        .unwrap_or_default()
                                }
                                on_unmaximize_window={
                                    custom_window_control_props
                                        .as_ref()
                                        .map(|props| props.on_unmaximize_window)
                                        .unwrap_or_default()
                                }
                                onClose_window={
                                    custom_window_control_props
                                        .as_ref()
                                        .map(|props| props.on_close_window)
                                        .unwrap_or_default()
                                }
                                AppBarComponent={AppBarComponent}
                            />
                        </PanelCatalogProvider>
                    </Suspense>
                </DndProvider>
            </MultiProvider>
        </MaybeLaunchPreference>
    }
}

fn context_menu_handler(event: MouseEvent) -> bool {
    if let Some(input_or_textarea) = event.target.downcast_ref::<Input>() || event.target.downcast_ref::<Textarea>() {
        return false;
    }

    event.preventDefault();
    true
}
```