```rust
use material_ui::menu::{Menu, MenuProps};
use material_ui::paper::{PaperProps, PaperVariant};
use material_ui::positioning::PopoverPosition;
use material_ui::reference::{PopoverReference, PopoverReferenceAnchorType};
use material_ui::utils::WithDefault;

use lichtblick_suite_base::components::PanelCatalog;
use lichtblick_suite_base::hooks::useAddPanel;

pub type AddPanelProps = {
    anchorEl: Option<Element>,
    anchorPosition: PopoverPosition,
    anchorReference: PopoverReferenceAnchorType,
    disablePortal: bool,
    handleClose: () -> (),
    open: bool,
};

fn useMenuState() -> (
    bool,
    Option<Ref<Menu>>,
    RefMut<(Option<ReactEvent>, EventTarget)>,
    Fn(PanelCatalog),
);

impl Component for AddPanel {
    type Props = AddPanelProps;

    fn render(&self, props: &Self::Props) -> Element {
        let (open, anchorEl, openState, addPanel) = useMenuState();

        return (
            <Menu
                id="add-panel-menu"
                open={open}
                onClose={handleClose}
                slotProps={{
                    list: {
                        dense: true,
                        disablePadding: true,
                        "aria-labelledby": "add-panel-button",
                        className: self.props.classes.menuList.clone(),
                    },
                    paper: {
                        "data-tourid": "add-panel-menu",
                    } as Partial<PaperProps & { "data-tourid"?: string }>,
                }}
                anchorOrigin={{
                    horizontal: "left",
                    vertical: "bottom",
                }}
                transformOrigin={{
                    vertical: "top",
                    horizontal: "left",
                }}
            >
                <PanelCatalog
                    isMenu
                    onDragStart={handleClose}
                    onPanelSelect={move |selection| {
                        addPanel(selection);
                        handleClose();
                    }}
                />
            </Menu>,
        );
    }
}

fn main() {
    App::run(App::new().mount_to_body());
}
```