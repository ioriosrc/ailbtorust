```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiagnosticStatusMsg {
    level: u8,
    name: String,
    hardware_id: String,
    message: String,
    values: Vec<KeyValue>,
}

#[derive(Serialize, Deserialize)]
pub struct KeyValue {
    key: String,
    value: String,
}

impl From<&DiagnosticStatusArrayMsg> for DiagnosticStatusMsg {
    fn from(status_array: &DiagnosticStatusArrayMsg) -> Self {
        DiagnosticStatusMsg {
            level: status_array.status[0].level,
            name: status_array.status[0].name.to_string(),
            hardware_id: status_array.status[0].hardware_id.to_string(),
            message: status_array.status[0].message.clone(),
            values: status_array
                .status
                .iter()
                .map(|msg| KeyValue {
                    key: msg.level.to_string(),
                    value: format!("{:.2}", msg.message.clone()),
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DiagnosticSummaryPanel<'a> {
    diagnostics: Vec<DiagnosticStatusMsg>,
    settings: &'a Settings,
    hardware_id_filter: String,
    pinned_ids: Vec<String>,
    sorted_by_level: bool,
    topic_to_render: String,
}

impl<'a> DiagnosticSummaryPanel<'a> {
    fn new(fixture: &Fixture) -> Self {
        let diagnostics = fixture
            .frame
            .get(&fixture.topics[0].name)
            .unwrap()
            .iter()
            .map(|msg| msg.clone())
            .collect();

        let settings = fixture.settings;

        DiagnosticSummaryPanel {
            diagnostics,
            settings,
            hardware_id_filter: fixture.fixture_data.hardwareIdFilter.to_string(),
            pinned_ids: fixture.fixture_data.pinnedIds.clone(),
            sorted_by_level: fixture.fixture_data.sortByLevel,
            topic_to_render: fixture.topics[0].name.to_string(),
        }
    }

    fn render(&self) -> String {
        let mut html = format!("<div>\n");

        for diagnostic in &self.diagnostics {
            if self.hardware_id_filter.is_empty() || diagnostic.hardware_id.contains(&self.hardware_id_filter) {
                let values_html = diagnostic
                    .values
                    .iter()
                    .map(|value| format!("{}: {}<br>", value.key, value.value))
                    .collect();

                html.push_str(&format!(
                    "<div class=\"diagnostic\" level=\"{:?}\">\n",
                    diagnostic.level
                ));

                if !self.pinned_ids.is_empty() && !self.pinned_ids.contains(diagnostic.hardware_id.as_str()) {
                    html.push_str("<div class=\"pin-icon\"></div>");
                }

                html.push_str(&format!(
                    "<h3>{}</h3>\n<code>{}</code><br>\n",
                    diagnostic.name,
                    diagnostic.message
                ));

                if !values_html.is_empty() {
                    html.push_str("<div class=\"value-list\">\n");
                    html.push_str(&values_html);
                    html.push_str("</div>");
                }

                html.push_str("</div>");
            }
        }

        html.push_str("</div>");

        html
    }
}

struct Settings {
    min_level: u8,
    hardware_id_filter: String,
    topic_to_render: String,
    sortByLevel: bool,
}
```