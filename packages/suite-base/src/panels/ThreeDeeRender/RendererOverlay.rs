```rust
useiced::widget::{self, mouse, scrollable};
useiced::{
    Application,
    Command,
    Element,
    KeyboardState,
    LayoutSize,
    Point,
    Size,
    TextStream,
    WinitEvent,
};

struct State {
    pub selected_renderables: Vec<PickedRenderable>,
    pub hovered_entities: Vec<HoverEntityInfo>,
    pub publish_menu_expanded: bool,
}

#[derive(Debug)]
enum Message {
    AddPanel(LayoutActions::AddPanel),
    ClickMeasure,
    ClickPublish,
    OnShowTopicSettings(String),
    OnTogglePerspective,
    PublishClickType(PublishClickType),
    ResetView,
}

struct RendererOverlay {
    state: State,
    theme:iced::theme::Theme,
}

impl Application for RendererOverlay {
    type Executor = iced::executor::Default;
    type Message = Message;

    fn new() -> Self {
        Self {
            state: Default::default(),
            theme: iced::theme::Theme::dark().with_accent_color(iced::Color::rgb8(13, 54, 129)),
        }
    }

    fn title(&self) -> String {
        "Renderer Overlay".to_string()
    }

    fn update(&mut self, ev: Event<Message>) -> Command<Message> {
        match ev {
            Event::KeyboardState(state) => {
                if state.pressed(iced::keyboard::KeyCode::Key3) && self.state.publish_menu_expanded {
                    self.state.publish_menu_expanded = false;
                }
            }
            Event::Mouse(mouse::Event::Wheel(_, scroll, _)) => {
                // Handle scrolling
            }
            Event::Mouse(mouse::Event::Button(MouseButton::Left, pressed)) => {
                if pressed {
                    // Handle left mouse click
                }
            }
            Event::Mouse(mouse::Event::Button(MouseButton::Right, pressed)) => {
                if pressed {
                    // Handle right mouse button press
                }
            }
            Event::Input(Input { stream }) => {
                if let Ok(event) = stream.next_textual() {
                    self.handle_text_event(event);
                }
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // Render the UI
    }

    fn theme(&self) -> &iced::theme::Theme {
        &self.theme
    }
}

fn main() -> iced::Result {
    Application::run(Settings {
        antialiasing: true,
        size: Size::new(800, 600),
        // other settings...
    })
}
```