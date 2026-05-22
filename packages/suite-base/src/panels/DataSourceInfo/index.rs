```rust
use std::collections::HashMap;

use crate::{
  messages::{MessagePipelineContext, Topic},
};

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use super::Panel;

const COPY_ICON_CLASS_NAME = "copy-icon";

struct SourceInfo {
  topics: HashMap<String, Topic>,
  start_time: Option<i64>,
  end_time: Option<i64>,
}

impl SourceInfo {
  fn new(topics: HashMap<String, Topic>, start_time: Option<i64>, end_time: Option<i64>) -> Self {
    SourceInfo {
      topics,
      start_time,
      end_time,
    }
  }

  fn render(&self) -> Result<Panel, String> {
    if self.start_time.is_none() || self.end_time.is_none() {
      return Err("Waiting for data…".to_string());
    }

    let mut table = "<table>\n";
    table.push_str("<thead>\n");
    table.push_str("<tr>\n");
    table.push_str("<th>Topic Name</th>\n");
    table.push_str("<th>Datatype</th>\n");
    table.push_str("<th>Message count</th>\n");
    table.push_str("<th>Frequency</th>\n");
    table.push_str("</tr>\n");
    table.push_str("</thead>\n");
    table.push_str("<tbody>");

    for (topic_name, topic) in &self.topics {
      table.push_str(&format!("<tr key=\"{}\">\n", topic_name));
      table.push_str(&format!(
        "<td>{}</td>\n",
        if topic.schema_name.is_none() {
          "—"
        } else {
          format!("{}{}", topic.schema_name, &topic.aliased_from_name.unwrap_or_default())
        }
      ));
      table.push_str("<td data-topic=\"{}\" data-topic-stat=\"count\">&mdash;</td>\n", topic_name);
      table.push_str("<td data-topic=\"{}\" data-topic-stat=\"frequency\">&mdash;</td>\n", topic_name);
      table.push_str("</tr>\n");
    }

    table.push_str("</tbody>");
    table.push_str("</table>");

    let direct_topic_stats_updater = format!(
      "<div style=\"display: block; flex: 1\">\n",
      &format!("<div class=\"copy-icon\" edge=\"end\" size=\"small\" iconSize=\"small\"></div>\n",),
      &format!(""),
      "Direct Topic Stats Updater: Interval={}</div>", 6
    );

    let panel = Panel::new(
      Some(table),
      None,
      Some(format!(
        "<p>Start Time: {}</p>\n",
        self.start_time.unwrap_or_default()
      )),
      Some(format!(
        "<p>End Time: {}</p>\n",
        self.end_time.unwrap_or_default()
      )),
    );

    Ok(panel)
  }
}

impl Panel for SourceInfo {
  fn panel_type(&self) -> &str {
    "SourceInfo"
  }

  fn default_config(&self) -> HashMap<&str, String> {
    HashMap::new()
  }
}
```