```rust
use iced::{
    Application, Clipboard, ClipboardEvent, Command, Element, Layout, Rectangle, Text,
};

struct App {
    message_paths: Vec<(String, bool)>,
    anchor_position: Option<Rectangle>,
}

impl Application for App {
    type State = ();
    type Event = ();

    fn update(&mut self, msg: Self::Event, state: &mut Self::State) {}

    fn view(&self) -> Element<Self> {
        let mut menu_items: Vec<Text> = self.message_paths.iter().map(|(path, is_topic)| {
            if !is_topic {
                Text::new(format!("Copy message path: {}", path))
            } else {
                Text::new(format!("Copy topic name: {}", path.split('/').nth_back(0).unwrap()))
            }
        }).collect();

        if self.message_paths.len() == 1 && self.message_paths[0].1 {
            menu_items.push(Text::new("Copy schema name"));
        }

        let items = menu_items.into_iter().map(|text| Element::new(text));

        Menu::new()
            .anchor(self.anchor_position)
            .items(items)
            .into()
    }
}

impl App {
    fn new(message_paths: Vec<(String, bool)>, anchor_position: Option<Rectangle>) -> Self {
        Self {
            message_paths,
            anchor_position,
        }
    }

    fn handle_clipboard_event(&mut self, event: ClipboardEvent) {
        if let Some(path) = self.message_paths.iter().find(|&(p, _)| p == &event.text()) {
            self.copy_to_clipboard(p);
        }
    }

    fn copy_to_clipboard(&self, path: &str) {
        // Implement the logic to copy text to clipboard
    }
}

fn main() {
    Application::run(
        App::new(vec![(String::from("path/to/message"), false), (String::from("topic1"), true)], None),
    );
}
```