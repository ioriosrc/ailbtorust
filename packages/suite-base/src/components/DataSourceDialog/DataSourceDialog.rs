```rust
use std::rc::Rc;

use mui_icons_material::{CloseIcon, CloseIconProps};
use mui_core::{IconButton, IconButtonProps, PaperProps};
use mui_material::stack::Stack;
use mui_material::dialog::{DialogProps, DialogState};
use mui_base::theme::create_theme;

use crate::{
    datasource_dialog::DataSourceDialogItem,
    player_selection::PlayerSelectionContext,
    workspace_store::WorkspaceStore,
    workspace_actions::WorkspaceActions,
    analytics::Analytics,
    app_event::AppEvent,
    DataSourceDialog as DataSourceDialogComponent,
};

pub type DataSourceDialogProps = {
    backdrop_animation: bool;
};

fn use_styles() -> Rc<super::theme::Theme> {
    create_theme()
}

fn select_data_source_dialog(store: WorkspaceStore) -> DialogState {
    store.dialogs.dataSource
}

pub fn DataSourceDialog(props: DataSourceDialogProps): Component<DataSourceDialogComponent> {
    let classes = useStyles();
    let { available_sources, select_source } = PlayerSelectionContext::use_context();
    let { dialog_actions } = WorkspaceActions::use_context();
    let active_data_source = use_workspace_store(select_data_source_dialog);
    let item: DataSourceDialogItem = active_data_source.item();

    let is_mounted = use_mounted_state();

    let first_sample_source = Rc::new(available_sources.iter().find(|source| source.type() == "sample").map(|&source| source.clone()).unwrap());

    let analytics = Analytics::use_context();

    let on_modal_close = {
        let analytics = Rc::clone(&analytics);
        move || {
            analytics.log_event(AppEvent::DIALOG_CLOSE, { active_data_source });
            dialog_actions.dataSource.close();
        }
    };

    let prev_active_view_ref = Rc::new(RefCell::<DataSourceDialogItem>::default());
    use_layout_effect(|| {
        if item == *prev_active_view_ref.borrow() {
            // Only run actions below when the active view actually changed
            return;
        }
        *prev_active_view_ref.borrow_mut() = item;
        if item == "file" {
            dialog_actions.open_file
                .open()
                .catch(|err| {
                    println!("Error: {}", err);
                })
                .finally(|| {
                    // set the view back to start so the user can click to open file again
                    if is_mounted() {
                        dialog_actions.dataSource.open("start");
                    }
                });
        } else if item == "demo" && first_sample_source.clone() != Rc::new(available_sources.iter().find(|source| source.type() == "sample").map(|&source| source.clone()).unwrap()) {
            select_source(first_sample_source);
        }
    }, [item, dialog_actions, first_sample_source, is_mounted, select_source]);

    let backdrop = Rc::new({
        if props.backdrop_animation == false {
            return None;
        } else if new Date().get_full_year() == 2025 && new Date().get_month() == 11 {
            Some(Snow { effect: "snow" })
        } else if new Date().get_full_year() == 2023 && new Date().get_month() == 0 {
            Some(Snow { effect: "confetti" })
        } else {
            None
        }
    });

    let view = Rc::new({
        match item {
            DataSourceDialogItem::Demo => {
                Some(DataSourceDialogComponent {
                    title: String::new(),
                    component: std::cell::Cell::<()>::default(),
                })
            }
            DataSourceDialogItem::Connection => {
                Some(DataSourceDialogComponent {
                    title: "Open new connection".to_string(),
                    component: Rc::new(Connection),
                })
            }
            _ => {
                Some(DataSourceDialogComponent {
                    title: "Get started".to_string(),
                    component: std::cell::Cell::<()>::default(),
                })
            }
        }
    });

    Component::view(|cx| {
        let theme = Rc::clone(&classes);

        let DialogProps {
            open,
            onClose,
            fullWidth,
            maxWidth,
            slotProps,
            ..DialogState::new()
        } = cx.use_context();

        let IconButtonProps {
            className,
            onClick,
            edge,
            ..IconButtonProps::new()
        } = cx.use_context();

        let PaperProps {
            square,
            elevation,
            className,
            style,
            ..PaperProps::new()
        } = cx.use_context();

        <Dialog
            data-testid="DataSourceDialog"
            open={open}
            onClose={onClose}
            fullWidth={fullWidth}
            maxWidth=maxWidth
            slotProps={{
                backdrop: {
                    children: backdrop.as_ref().map(|backdrop| backdrop.render(cx)),
                },
                paper: {
                    square: square,
                    elevation: elevation,
                    className: cx.merge_classes(classes.paper),
                    style: Rc::new({
                        overflow: item == "connection" && !props.backdrop_animation => "hidden",
                    }),
                },
            }}
        >
            <IconButton
                className={cx.merge_classes(classes.closeButton)}
                onClick={onClose}
                edge={edge}
            >
                <CloseIcon data-testid="CloseIcon" />
            </IconButton>
            <Stack
                flexGrow={1}
                fullHeight
                justifyContent="space-between"
                overflow=item == "connection" && !props.backdrop_animation => "hidden",
            >
                {view.render(cx)}
            </Stack>
        </Dialog>
    })
}

pub fn use_mounted_state() -> Rc<RefCell<bool>> {
    Rc::new(RefCell::<bool>::default())
}
```