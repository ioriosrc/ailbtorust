```rust
use std::rc::{Rc, RefCell};
use std::cell::Ref;
use std::collections::HashSet;

use crate::context::{
    ICurrentLayout,
    LayoutID,
    LayoutState,
    SelectedLayout,
};

use crate::actions::{
    PanelsActions,
    defaultPlaybackConfig,
};

#[derive(Clone)]
struct MockCurrentLayout {
    layout_state: RefCell<LayoutState>,
    add_layout_state_listener: Rc<dyn Fn(&mut LayoutState) -> ()>,
    remove_layout_state_listener: Rc<dyn Fn(&mut LayoutState) -> ()>,
}

impl Default for MockCurrentLayout {
    fn default() -> Self {
        let listeners = HashSet::new();
        let add_listener = Rc::new(move |layout| {
            listeners.insert(layout);
        });
        let remove_listener = Rc::new(move |layout| {
            listeners.remove(layout);
        });

        Self {
            layout_state: RefCell::new(LayoutState {
                selected_layout: SelectedLayout {
                    id: "mock-layout" as LayoutID,
                    data: {
                        config_by_id: {},
                        global_variables: {},
                        user_nodes: {},
                        playback_config: defaultPlaybackConfig,
                        ..Default::default(),
                    },
                },
            }),
            add_layout_state_listener,
            remove_layout_state_listener,
        }
    }
}

impl MockCurrentLayout {
    fn set_layout_state(&self, new_state: LayoutState) {
        *self.layout_state.borrow_mut() = new_state;

        // listeners rely on being able to getCurrentLayoutState() inside effects that may run before we re-render
        let state = self.layout_state.clone();
        self.add_layout_state_listener.clone().unwrap()(state);

        for listener in &listeners {
            listener(&state);
        }
    }

    fn set_current_layout(&self, new_layout: SelectedLayout) {
        self.set_layout_state(LayoutState {
            selected_layout: new_layout,
        });
    }

    fn update_shared_panel_state(&self, type_: &'static str, new_shared_state: String) {
        let mut state = self.layout_state.borrow_mut();
        if let Some(ref shared_panel_state) = &mut state.shared_panel_state {
            shared_panel_state.insert(type_, new_shared_state);
        }
    }

    fn perform_action(&self, action: PanelsActions) {
        // onAction?.(action);
        self.set_layout_state(LayoutState {
            selected_layout: Self::get_selected_layout(),
            ..Default::default()
        });
    }
}

pub struct CurrentLayoutProvider<'a> {
    children: &'a ReactNode,
    initial_state: Option<Partial<LayoutData>>,
    on_action: Option<&'a dyn Fn(PanelsActions)>,
}

impl<'a> Component for CurrentLayoutProvider<'a> {
    type Props = CurrentLayoutProviderProps;

    fn render(&self) -> JSXElement {
        let children = self.children;
        let initial_state = self.initial_state;
        let on_action = self.on_action;

        let layout_state_listeners = Rc::new(RefCell::new(Vec::new()));

        let add_layout_state_listener = move |listener: &mut LayoutState| {
            layout_state_listeners.borrow_mut().push(listener);
        };

        let remove_layout_state_listener = move |listener: &mut LayoutState| {
            layout_state_listeners.borrow_mut().retain(|&l| l != listener);
        };

        let layout_state_ref = Rc::new(RefCell::new(LayoutState {
            selected_layout: SelectedLayout {
                id: "mock-layout" as LayoutID,
                data: {
                    config_by_id: {},
                    global_variables: {},
                    user_nodes: {},
                    playback_config: defaultPlaybackConfig,
                    ..initial_state.unwrap_or_default(),
                },
            },
        }));

        let set_layout_state = move |new_state| {
            *layout_state_ref.borrow_mut() = new_state;

            // listeners rely on being able to getCurrentLayoutState() inside effects that may run before we re-render
            let state = layout_state.clone();
            add_layout_state_listener(&state);

            for listener in &listeners.borrow() {
                listener(&state);
            }
        };

        let setCurrent_layout = move |new_layout| {
            set_layout_state(LayoutState {
                selected_layout: new_layout,
            });
        };

        let update_shared_panel_state = move |type_: &'static str, new_shared_state: String| {
            let mut state = layout_state.borrow_mut();
            if let Some(ref shared_panel_state) = &mut state.shared_panel_state {
                shared_panel_state.insert(type_, new_shared_state);
            }
        };

        let perform_action = move |action: PanelsActions| {
            // onAction?.(action);
            set_layout_state(LayoutState {
                selected_layout: Self::get_selected_layout(),
                ..Default::default()
            });
        };

        let actions = Rc::new(MockCurrentLayout {
            layout_state: layout_state_ref.clone(),
            add_layout_state_listener,
            remove_layout_state_listener,
        });

        <CurrentLayoutContext.Provider value={{ ...actions }}>{children}</CurrentLayoutContext.Provider>
    }
}
```