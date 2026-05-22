```rust
use std::rc::Rc;

use crate::{components, types, utils};
useiced::Command;
use iced::{
    container,
    element,
    layout,
    mouse,
    widget,
    Application,
    ButtonBase,
    Column,
    Element,
    Image,
    Row,
    ScreenSize,
    Size,
    Text,
};

#[derive(Debug)]
pub struct TabbedToolbar {
    panel_id: String,
    actions: types::TabActions,
    tabs: Vec<types::TabConfig>,
    active_tab_idx: usize,
    is_over_dragging_tab: bool,
    dragging_tab_item: Option<Rc<dyn DraggingTabItem>>,
}

impl TabbedToolbar {
    pub fn new(panel_id: String, actions: types::TabActions, tabs: Vec<types::TabConfig>, active_tab_idx: usize) -> Self {
        Self {
            panel_id,
            actions,
            tabs,
            active_tab_idx,
            is_over_dragging_tab: false,
            dragging_tab_item: None,
        }
    }

    fn handle_input(&mut self, event: mouse::Event) -> Command<()> {
        if let Some(dragging_tab_item) = &self.dragging_tab_item {
            match dragging_tab_item.on_click(event) {
                Ok(_) => {
                    self.is_over_dragging_tab = false;
                    self.dragging_tab_item = None;
                }
                Err(err) => error!("{:?}", err),
            }
        }

        if event.kind == mouse::EventKind::DoubleClick {
            self.actions.add_tab(&self.panel_id);
        }

        Command::none()
    }

    fn handle_message(&mut self, message: Message) -> Command<()> {
        match message {
            Message::AddTab => {
                let mut new_tabs = Vec::with_capacity(self.tabs.len() + 1);
                new_tabs.extend_from_slice(&self.tabs);
                new_tabs.push(types::TabConfig::default());
                self.tabs = new_tabs;
                self.active_tab_idx = new_tabs.len() - 1;
            }
            Message::DragOver(item) => {
                if item.panel_id == self.panel_id {
                    self.is_over_dragging_tab = true;
                    self.dragging_tab_item = Some(Rc::new(item));
                }
            },
            Message::Drop(_) => {
                self.is_over_dragging_tab = false;
                self.dragging_tab_item = None;
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, ()> {
        let panel_title = Text::new(self.tabs.get(self.active_tab_idx).unwrap().title()).size(20);

        let tab_container = match self.is_over_dragging_tab {
            true => container::Container::new(
                column![
                    Row![panel_title, Text::new("Drag and drop a new tab here!")],
                    Space::with_height(self.tabs.len() as f32 * 20.0),
                ],
            )
            .padding(10)
            .style(|theme| theme.panel_toolbar().container()),
            false => container::Container::new(
                column![
                    panel_title,
                    TabList {
                        actions: Rc::clone(&self.actions),
                        tabs: Rc::clone(&self.tabs),
                        active_tab_idx: self.active_tab_idx,
                    },
                ],
            )
            .padding(10)
            .style(|theme| theme.panel_toolbar().container()),
        };

        container::Container::new(
            row![tab_container, ButtonBase::new("Add tab").on_click(self.actions.add_tab)]
                .padding(5),
        )
        .style(|theme| theme.panel_toolbar())
    }
}

impl Application for TabbedToolbar {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Tile = Size;

    fn title(&self) -> String {
        "Tabbed Toolbar".to_string()
    }

    fn bootstrap(_init: Self::Init) -> Command<Self::Message> {
        Command::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut command = Command::none();

        if let Message::DragOver(item) = &message {
            if item.panel_id == self.panel_id {
                command.push_back(Message::DragOver(item.clone()));
            }
        }

        if let Message::Drop(_) = &message {
            command.push_back(Message::Drop(Rc::clone(&self.dragging_tab_item)));
        }

        command.push_back(self.handle_message(message));

        command
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.view()
    }
}

impl TabList {
    pub fn new(
        actions: Rc<dyn DraggingTabItem>,
        tabs: Rc<Vec<types::TabConfig>>,
        active_tab_idx: usize,
    ) -> Self {
        TabList {
            actions,
            tabs,
            active_tab_idx,
        }
    }
}

#[derive(Clone, Debug)]
struct DraggingTabItem {
    item: Rc<dyn TabDndContext>,
    is_over: bool,
}
```