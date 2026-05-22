```rust
use html5ever::{serialize_into_string, local_name};
use html5ever::parse;
use html5ever::tree_builder::{
    tree_builder::TreeBuilder,
    tree_builder::NodeData,
    tree_builder::NodeKind,
};
use html5ever::tree_builder::ProcessDocument;
use std::collections::HashSet;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0
#[derive(Debug)]
struct SanitizedHtml {
    html: String,
}

fn sanitize(value: &str) -> SanitizedHtml {
    let mut allowed_tags = HashSet::from([
        local_name!("b"),
        local_name!("i"),
        local_name!("u"),
        local_name!("strong"),
        local_name!("em"),
        local_name!("h1"),
        local_name!("h2"),
        local_name!("h3"),
        local_name!("h4"),
        local_name!("h5"),
        local_name!("p"),
        local_name!("a"),
    ]);

    let mut allowed_attributes = HashSet::from([
        ("color", "String"),
        ("size", "String"),
        ("colspan", "u32"),
    ]);

    let mut tree_builder = TreeBuilder::new();
    tree_builder.parse_fragment(
        &[
            b"<!DOCTYPE html>",
            b"",
            value.as_bytes(),
            b"",
        ]
        .iter()
        .cloned()
        .collect::<Vec<_>>()
        .as_slice(),
        false,
        false,
    );

    let mut sanitized_html = String::new();
    for (data, _child) in tree_builder.into_nodes() {
        match data {
            NodeData::Text { text } => {
                sanitized_html.push_str(text);
            }
            NodeData::Element { name, attrs } => {
                if allowed_tags.contains(&name) {
                    let mut attr_string = String::new();
                    for (attr_name, attr_value) in attrs {
                        attr_string.push_str(format!(" {}=\"{}\"", attr_name, attr_value).as_str());
                    }
                    sanitized_html.push_str(&format!("<{}", name.as_str(), attr_string));
                    for child in tree_builder.into_children() {
                        match child {
                            NodeData::Text { text } => sanitized_html.push_str(text),
                            _ => continue,
                        }
                    }
                    sanitized_html.push_str("</{}>", name.as_str());
                } else {
                    sanitized_html.push_str("<!-- Ignored element: {}", name.as_str());
                }
            }
            _ => continue,
        }
    }

    SanitizedHtml { html: sanitized_html }
}
```