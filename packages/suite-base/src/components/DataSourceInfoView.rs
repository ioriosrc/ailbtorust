```rust
use std::rc::Rc;
use tui::{
    backend::TermionBackend,
    Terminal,
    widgets::{Box, Text},
};
use crate::message_pipeline::MessagePipelineContext;
use crate::players::PlayerPresence;

pub struct DataSourceInfoContent {
    disable_source: bool,
    duration_ref: Rc<dyn std::any::Any>,
    endTime_ref: Rc<dyn std::any::Any>,
    player_name: Option<String>,
    player_presence: PlayerPresence,
    start_time: Option<crate::suite_base::Time>,
    is_live_connection: bool,
}

impl DataSourceInfoContent {
    pub fn new(
        disable_source: bool,
        duration_ref: Rc<dyn std::any::Any>,
        endTime_ref: Rc<dyn std::any::Any>,
        player_name: Option<String>,
        player_presence: PlayerPresence,
        start_time: Option<crate::suite_base::Time>,
        is_live_connection: bool,
    ) -> Self {
        Self {
            disable_source,
            duration_ref,
            endTime_ref,
            player_name,
            player_presence,
            start_time,
            is_live_connection,
        }
    }

    pub fn render(&self, terminal: &mut Terminal<TermionBackend>) {
        if !self.disable_source {
            if self.player_presence == PlayerPresence::INITIALIZING {
                Text::from("Loading...")
                    .color(tui::colors::LightBlue)
                    .render(terminal);
            } else if self.player_presence == PlayerPresence::RECONNECTING {
                Text::from("Waiting for connection")
                    .color(tui::colors::Yellow)
                    .render(terminal);
            } else if let Some(player_name) = &self.player_name {
                Text::from(&player_name)
                    .style(tui::style::Color::LightBlue)
                    .render(terminal);
            } else {
                Text::from("...")
                    .color(tui::colors::LightBlue)
                    .render(terminal);
            }
        }

        if let Some(start_time) = &self.start_time {
            if self.player_presence == PlayerPresence::INITIALIZING {
                Text::from("Loading...")
                    .color(tui::colors::LightBlue)
                    .render(terminal);
            } else {
                let date = format!("{:?}", start_time);
                let time = if is_absolute_time(start_time) {
                    format!(" - {}", format!("{:?}", start_time))
                } else {
                    " - "
                };
                Text::from(&date.to_string() + time)
                    .style(tui::style::Color::LightBlue)
                    .render(terminal);
            }
        }

        if !self.is_live_connection && let Some(end_time) = &self.end_time {
            if self.player_presence == PlayerPresence::INITIALIZING {
                Text::from("Loading...")
                    .color(tui::colors::LightBlue)
                    .render(terminal);
            } else {
                let date = format!("{:?}", end_time);
                let time = if is_absolute_time(end_time) {
                    format!(" - {}", format!("{:?}", end_time))
                } else {
                    " - "
                };
                Text::from(&date.to_string() + time)
                    .style(tui::style::Color::LightBlue)
                    .render(terminal);
            }
        }

        if let Some(start_time) = &self.start_time {
            if self.player_presence == PlayerPresence::INITIALIZING {
                Text::from("Loading...")
                    .color(tui::colors::LightBlue)
                    .render(terminal);
            } else {
                let duration = start_time.duration_since(&start_time).as_secs_f64();
                let formatted_duration = format!("{:?}", duration);
                Text::from(&formatted_duration.to_string())
                    .style(tui::style::Color::LightBlue)
                    .render(terminal);
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new(TermionBackend::stdout()).unwrap();
    DataSourceInfoContent::new(
        false,
        Rc::from(std::any::Any::try_from("Player1").unwrap()),
        Rc::from(std::any::Any::try_from("2023-04-01T12:00:00Z").unwrap()),
        Some(String::from("John Doe")),
        PlayerPresence::ACTIVE,
        Some(crate::suite_base::Time::from_seconds_f64(3600.0)),
        false,
    )
    .render(&mut terminal);
}
```