```rust
use crate::components::stack::Stack;
use crate::{ErrorDisplayProps, ErrorInfo, SanitizeStack};
use mui_base::prelude::*;
use mui_material::{Typography, Link, Divider};
use react::jsx;

use std::fmt::Debug;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

fn sanitize_stack(stack: &str) -> String {
    let mut lines = stack.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("at ") || line.contains("https://") {
            break;
        }
    }

    // Remove the first empty line and trim leading/trailing spaces
    lines.next().map_or(String::from(""), |line| format!("  {}", line.trim()))
}

fn ErrorStacktrace(props: ErrorStackProps) -> Jsx<Html> {
    let classes = use_styles();

    let stack_without_message = props.stack.to_string();
    if stack_without_message.starts_with(&props.error.message) {
        stack_without_message.remove(0..props.error.message.len());
    }

    let error_details = Some(
        <div>
            <Typography fontWeight="bold">Error stack:</Typography>
            <ErrorStacktrace
                stack={sanitize_stack(&stack_without_message)}
                hide_source_locations={props.hide_error_source_locations}
            />
            {props.error_info.map(|error_info| {
                <Typography fontWeight="bold">Component stack:</Typography>
                <ErrorStacktrace
                    stack=format!("  {}", error_info.component_stack)
                        .trim()
                        .to_string()
                />
            })}
        </div>,
    );

    jsx! {{
        <Stack gap={2} paddingBottom={2}>
            <Stack>
                <Typography variant="h4" gutterBottom>
                    {props.title.unwrap_or("The app encountered an unexpected error")}
                </Typography>
                <Typography variant="body1">{props.content}</Typography>
            </Stack>
            <Divider />
            <Typography variant="subtitle2" component="code" fontWeight="bold">
                {props.error.message}
            </Typography>
            <Link
                color="secondary"
                onClick={() => {
                    props.set_show_error_details(!props.show_error_details);
                }}
            >
                {props.show_error_details ? "Hide" : "Show"} details
            </Link>
        </Stack>
        {error_details}
    }}
}

#[derive(Debug, Clone)]
struct ErrorDisplayProps {
    title: Option<String>,
    error: Option<Error>,
    error_info: Option<ErrorInfo>,
    content: Jsx<Html>,
    actions: Jsx<Html>,
    show_error_details: bool,
    hide_error_source_locations: bool,
}

fn main() {}
```