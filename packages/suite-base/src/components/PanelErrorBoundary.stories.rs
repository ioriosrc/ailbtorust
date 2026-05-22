```rust
use std::error;
use std::fmt;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use storytale::{Storybook, StoryObj};

#[derive(Debug)]
struct PanelErrorBoundaryError {
    message: String,
    stacktrace: Option<String>,
}

impl fmt::Display for PanelErrorBoundaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(stack) = &self.stacktrace {
            writeln!(f, " Stack trace:\n{}", stack)?;
        }
        Ok(())
    }
}

impl error::Error for PanelErrorBoundaryError {}

fn broken() -> Result<(), Box<dyn error::Error>> {
    Err(Box::new(PanelErrorBoundaryError {
        message: String::from("Hello!"),
        stacktrace: Some(String::from("
an error occurred
it's caught by this component
now the user sees
")),
    }))
}

fn main() {
    let mut storybook = Storybook::new("components/PanelErrorBoundary");

    storybook.add_story(
        "Default",
        |args, env| {
            Box::pin(async move {
                <DndProvider backend=HTML5Backend>>
                    <PanelErrorBoundary
                        on_remove_panel=action("onRemovePanel")
                        on_reset_panel=action("onResetPanel")
                    >
                        <Broken />
                    </PanelErrorBoundary>
                </DndProvider>
            })
        },
    );

    storybook.add_story(
        "ShowingDetails",
        |args, env| {
            Box::pin(async move {
                <DndProvider backend=HTML5Backend>>
                    <PanelErrorBoundary
                        show_error_details
                        hide_error_source_locations
                        on_remove_panel=action("onRemovePanel")
                        on_reset_panel=action("onResetPanel")
                    >
                        <Broken />
                    </PanelErrorBoundary>
                </DndProvider>
            })
        },
    );

    storybook.run();
}
```