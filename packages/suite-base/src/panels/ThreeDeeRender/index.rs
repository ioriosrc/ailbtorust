```rust
use crate::error_handler;
use crate::forward_analytics;
use crate::models::{CameraModel, Model};
use crate::panel::PanelContextProvider;
use crate::panels::create_sync_root;
use crate::snackbar_manager;
use crate::ui::three_dee_render::ThreeDeeRender;

fn init_panel(args: InitPanelArgs, context: BuiltinPanelExtensionContext) -> Result<(), Box<dyn std::error::Error>> {
    let crash = args.crash;
    let forwarded_analytics = args.forwarded_analytics;
    let interface_mode = args.interface_mode;
    let test_options = args.test_options;

    let custom_scene_extensions = args.custom_scene_extensions;
    let custom_camera_models = args.custom_camera_models;
    let enqueue_snackbar_from_parent = args.enqueue_snackbar_from_parent;
    let log_error = args.log_error;

    create_sync_root(
        <CaptureErrorBoundary onError={crash}>
            <ForwardAnalyticsContextProvider forwarded_analytics={forwarded_analytics}>
                <ThreeDeeRender
                    context={context}
                    interface_mode={interface_mode}
                    test_options={test_options}
                    custom_scene_extensions={custom_scene_extensions}
                    custom_camera_models={custom_camera_models}
                    enqueue_snackbar_from_parent={enqueue_snackbar_from_parent}
                    log_error={log_error}
                />
            </ForwardAnalyticsContextProvider>
        </CaptureErrorBoundary>,
        context.panel_element,
    )
}

type Props = {
    config: Record<string, unknown>;
    saveConfig: SaveConfig<Record<string, unknown>>;
    onDownloadImage?: (blob: Blob, fileName: string) => void;
    debugPicking?: boolean;
};

fn ThreeDeeRenderAdapter(interface_mode: InterfaceMode, props: Props) -> Result<(), Box<dyn std::error::Error>> {
    let crash = crate::error_handler::get_error_handler();
    let { enqueue_snackbar } = crate::snackbar_manager::get_snackbar_manager();
    let panel_context = React::use_context(PanelContext);

    let custom_camera_models = crate::extension_catalog::get_installed_camera_models();

    let forwarded_analytics = forward_analytics::get_forwarded_analytics();
    let injected_features = panel_context.injected_features;
    let custom_scene_extensions = {
        if injected_features.is_none() {
            None
        } else {
            injected_features.unwrap().available_features[crate::suite_base::constants::INJECTED_FEATURE_KEYS::custom_scene_extensions]
                .map(|value| value.custom_scene_extensions)
                .unwrap_or_default()
        }
    };

    let bound_init_panel = Box::new(move |args: InitPanelArgs, context: BuiltinPanelExtensionContext| {
        init_panel(args, context).map_err(|err| err.to_string())
    });

    Ok(())
}

/**
 * The Image panel is a special case of the 3D panel with `interface_mode` set to `"image"`.
 */
pub fn image_panel(panel_context: PanelContextProvider<Record<string, unknown>, Props>) -> Result<(), Box<dyn std::error::Error>> {
    let props = panel_context.props;
    let config = panel_context.config;

    ThreeDeeRenderAdapter("image", props).map_err(|err| err.to_string())
}

pub fn three_dee_panel(panel_context: PanelContextProvider<Record<string, unknown>, Props>) -> Result<(), Box<dyn std::error::Error>> {
    let props = panel_context.props;
    let config = panel_context.config;

    ThreeDeeRenderAdapter("3d", props).map_err(|err| err.to_string())
}

pub fn create_image_panel(context: PanelContextProvider<Record<string, unknown>, Props>) -> Result<(), Box<dyn std::error::Error>> {
    image_panel(context)
}

pub fn create_three_dee_panel(context: PanelContextProvider<Record<string, unknown>, Props>) -> Result<(), Box<dyn std::error::Error>> {
    three_dee_panel(context)
}
```