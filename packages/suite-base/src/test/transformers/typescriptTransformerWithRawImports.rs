```rust
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_transformer() -> impl Fn(&str, &str, &babel_jest::Config) -> babel_jest::TransformResult {
  Box::new(move |source_text, source_path, opt| {
    let mut rewritten_source = source_text.clone();
    let import_reg_ex = regex::Regex::new(r"^import (.*) from \"(.*?)\"?\?raw;$").unwrap();

    if import_reg_ex.is_match(source_text) {
      for cap in import_reg_ex.captures_iter(&source_text) {
        let p1: String = cap[1].to_string();
        let p2: String = cap[2].to_string();

        let resolved_path = path::PathBuf::new()
          .join(p2)
          .parent()
          .unwrap()
          .join(&p1);

        let raw_file_content = fs::read_to_string(resolved_path).expect("Failed to read file");
        rewritten_source = rewritten_source.replace(&format!("import {p1} from \"{p2}\"?\?raw;"), &format!("const {p1} = {};", &raw_file_content));
      }
    }

    babel_jest::TransformResult {
      code: rewritten_source,
      map: None,
      filename: Some(source_path.to_string()),
    }
  })
}
```