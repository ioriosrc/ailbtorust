```rust
use std::rc::Rc;

pub type PanelType = String;

pub type SharedPanelState = Vec<f64>; // Example shared state, replace with actual data

#[derive(Clone)]
pub struct LayoutData {
    layout: Option<Vec<PanelConfig>>,
}

impl LayoutData {
    pub fn new(layout: Option<Vec<PanelConfig>>) -> Self {
        LayoutData { layout }
    }
}

pub type LayoutID = String;

pub type LayoutState = Rc<{
    shared_panel_state: Option<Record<PanelType, SharedPanelState>>,
    selected_layout: Option<{
        id: LayoutID;
        loading?: bool;
        data: LayoutData | None;
        name?: Option<String>;
        edited?: bool;
    }>,
}>;

pub trait CurrentLayoutProvider {
    fn add_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>);
    fn remove_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>) -> Result<(), Box<dyn std::error::Error>>;
    fn add_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>);
    fn remove_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>);

    fn mosaic_id(&self) -> String;
}

pub struct CurrentLayout {
    shared_panel_state: Option<Record<PanelType, SharedPanelState>>,
    selected_layout: Option<{
        id: LayoutID;
        loading?: bool;
        data: LayoutData | None;
        name?: Option<String>;
        edited?: bool;
    }>,
    mosaic_id: String,
}

impl CurrentLayoutProvider for CurrentLayout {
    fn add_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>) {
        // Implementation to add layout state listener
    }

    fn remove_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation to remove layout state listener
        Ok(())
    }

    fn add_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>) {
        // Implementation to add selected panel IDs listener
    }

    fn remove_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation to remove selected panel IDs listener
        Ok(())
    }

    fn mosaic_id(&self) -> String {
        self.mosaic_id.clone()
    }
}

fn get_leaves(layout_data: Option<&Vec<PanelConfig>>) -> Vec<String> {
    layout_data.map_or(vec![], |layout| {
        let mut leaves = vec![];
        for panel in layout {
            leaves.push(panel.name.clone());
        }
        leaves
    })
}

pub trait CurrentLayoutActions {
    fn add_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>);
    fn remove_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>) -> Result<(), Box<dyn std::error::Error>>;
    fn add_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>);
    fn remove_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>);

    fn get_current_layout_state(&self) -> LayoutState;
    fn set_selected_layout_id(&mut self, id: LayoutID);
    fn update_shared_panel_state(&self, type_: PanelType, data: SharedPanelState);
    fn save_panel_configs(&self, payload: SaveConfigsPayload);
    fn update_panel_configs(&self, panel_type: String, updater: impl Fn(&PanelConfig) -> PanelConfig);
    fn create_tab_panel(&self, payload: CreateTabPanelPayload);
    fn change_panel_layout(&self, payload: ChangePanelLayoutPayload);
    fn overwrite_global_variables(&mut self, payload: Record<String, VariableValue>);
    fn set_global_variables(&mut self, payload: Record<String, VariableValue>);
    fn setUserScripts(&mut self, payload: Partial<UserScripts>);
    fn set_playback_config(&mut self, payload: Partial<PlaybackConfig>);
    fn close_panel(&self, payload: ClosePanelPayload);
    fn split_panel(&self, payload: SplitPanelPayload);
    fn swap_panel(&self, payload: SwapPanelPayload);
    fn move_tab(&self, payload: MoveTabPayload);
    fn add_panel(&self, payload: AddPanelPayload);
    fn drop_panel(&self, payload: DropPanelPayload);
    fn start_drag(&self, payload: StartDragPayload);
    fn end_drag(&self, payload: EndDragPayload);
}

pub struct CurrentLayoutImpl {
    shared_panel_state: Option<Record<PanelType, SharedPanelState>>,
    selected_layout: Option<{
        id: LayoutID;
        loading?: bool;
        data: LayoutData | None;
        name?: Option<String>;
        edited?: bool;
    }>,
    mosaic_id: String,
}

impl CurrentLayoutImpl {
    pub fn new(mosaic_id: String) -> Self {
        CurrentLayoutImpl {
            shared_panel_state: None,
            selected_layout: None,
            mosaic_id,
        }
    }

    // Implement methods for layout state listeners, selected panel IDs listeners, and actions

    fn add_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>) {
        // Implementation to add layout state listener
    }

    fn remove_layout_state_listener(&self, listener: Box<dyn FnMut(LayoutState)>) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation to remove layout state listener
        Ok(())
    }

    fn add_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>) {
        // Implementation to add selected panel IDs listener
    }

    fn remove_selected_panel_ids_listener(&self, listener: Box<dyn FnMut(Vec<LayoutID>)>) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation to remove selected panel IDs listener
        Ok(())
    }

    fn get_current_layout_state(&self) -> LayoutState {
        LayoutState {
            shared_panel_state: self.shared_panel_state.clone(),
            selected_layout: self.selected_layout.clone(),
        }
    }

    fn set_selected_layout_id(&mut self, id: LayoutID) {
        self.selected_layout = Some({
            id,
            loading: false,
            data: None,
            name: None,
            edited: false,
        });
    }

    fn update_shared_panel_state(&self, type_: PanelType, data: SharedPanelState) {
        if let Some(shared_state) = &mut self.shared_panel_state {
            shared_state.insert(type_, data);
        }
    }

    fn save_panel_configs(&self, payload: SaveConfigsPayload) {
        // Implementation to save panel configurations
    }

    fn update_panel_configs(&self, panel_type: String, updater: impl Fn(&PanelConfig) -> PanelConfig) {
        if let Some(layout_data) = &mut self.selected_layout.as_mut().and_then(|layout| &mut layout.data) {
            if let Some(panel_config) = layout_data.config_by_id.get_mut(&panel_type) {
                *panel_config = updater(panel_config);
            }
        }
    }

    fn create_tab_panel(&self, payload: CreateTabPanelPayload) {
        // Implementation to create a tab panel
    }

    fn change_panel_layout(&self, payload: ChangePanelLayoutPayload) {
        // Implementation to change the layout of a panel
    }

    fn overwrite_global_variables(&mut self, payload: Record<String, VariableValue>) {
        // Implementation to overwrite global variables
    }

    fn set_global_variables(&mut self, payload: Record<String, VariableValue>) {
        // Implementation to set global variables
    }

    fn setUserScripts(&mut self, payload: Partial<UserScripts>) {
        // Implementation to set user scripts
    }

    fn set_playback_config(&mut self, payload: Partial<PlaybackConfig>) {
        // Implementation to set playback configuration
    }

    fn close_panel(&self, payload: ClosePanelPayload) {
        // Implementation to close a panel
    }

    fn split_panel(&self, payload: SplitPanelPayload) {
        // Implementation to split a panel
    }

    fn swap_panel(&self, payload: SwapPanelPayload) {
        // Implementation to swap panels
    }

    fn move_tab(&self, payload: MoveTabPayload) {
        // Implementation to move a tab
    }

    fn add_panel(&self, payload: AddPanelPayload) {
        // Implementation to add a panel
    }

    fn drop_panel(&self, payload: DropPanelPayload) {
        // Implementation to drop a panel
    }

    fn start_drag(&self, payload: StartDragPayload) {
        // Implementation to start dragging
    }

    fn end_drag(&self, payload: EndDragPayload) {
        // Implementation to end dragging
    }
}

pub struct CurrentLayoutContext(pub Rc<CurrentLayoutImpl>);

impl Default for CurrentLayoutContext {
    fn default() -> Self {
        Self(Rc::new(CurrentLayoutImpl::new("default_id")))
    }
}
```