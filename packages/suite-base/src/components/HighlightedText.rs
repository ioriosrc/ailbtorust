```rust
use std::cmp::{Ordering};
use std::fmt;

#[derive(Debug)]
pub struct HighlightedText {
    text: String,
    highlight: Option<String>,
}

impl HighlightedText {
    pub fn new(text: &str, highlight: Option<&str>) -> Self {
        Self {
            text: text.to_string(),
            highlight: highlight.map(|s| s.to_string()),
        }
    }

    fn render(&self) -> String {
        if self.highlight.is_none() || !self.highlight.as_ref().unwrap().trim().is_empty() {
            return self.text.clone();
        }

        let regex = Regex::new(self.highlight.as_ref().unwrap()).expect("Invalid highlight");
        let parts: Vec<&str> = regex.split(&self.text).collect();

        let mut result = String::new();
        for (i, part) in parts.iter().enumerate() {
            if i == 0 || !regex.is_match(part) {
                result.push_str(part);
            } else {
                result.push('<mark>');
                result.push_str(part);
                result.push('</mark>');
            }
        }

        result
    }
}

fn main() {
    let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let highlight = Some("ipsum");
    let highlighted_text = HighlightedText::new(text, highlight);
    println!("{}", highlighted_text.render());
}
```