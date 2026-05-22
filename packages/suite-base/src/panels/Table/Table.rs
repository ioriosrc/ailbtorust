```rust
use std::collections::HashMap;

fn is_typed_array(value: &dyn std::any::Any) -> bool {
  match value.downcast_ref::<Vec<isize>>() {
    Some(vec) => !vec.is_empty(),
    None => false,
  }
}

type CellValue = dyn std::any::Any;
type MergedColumnsType = HashMap<String, TableCell>;

fn get_columns_from_object(val: &CellValue, accessor_path: &str) -> Vec<MergedColumnsType> {
  if is_typed_array(&val) {
    vec![MergedColumnsType::new(
      "typedArray".to_string(),
      TableCell::new("".to_string()),
    )]
  } else {
    let mut columns = val
      .downcast_ref::<HashMap<String, CellValue>>()
      .unwrap_or(&HashMap::new())
      .iter()
      .map(|(accessor, value)| {
        let id = if accessor_path.is_empty() {
          accessor.to_string()
        } else {
          format!("{}.{}", accessor_path, accessor)
        };
        MergedColumnsType::new(accessor.to_string(), TableCell::new(value.clone()))
      })
      .collect();

    if accessor_path.is_empty() {
      columns.insert(
        "expander".to_string(),
        TableCell::new("".to_string()),
      );
    }

    columns
  }
}

struct TableCell {
  value: CellValue,
}

impl TableCell {
  fn new(accessor: String) -> Self {
    TableCell { value: accessor }
  }
}

fn text_cell_content(props: &TextCellContentProps) -> String {
  props.value.to_string()
}

struct TextCellContentProps {
  value: String;
}

pub struct Table {
  value: CellValue,
  accessor_path: String,
}

impl Table {
  fn new(value: CellValue, accessor_path: String) -> Self {
    Self { value, accessor_path }
  }

  fn render(&self) -> String {
    if !is_typed_array(&self.value) && self.value != null {
      let columns = get_columns_from_object(&self.value, &self.accessor_path);
      return table::render_table(columns, self.get_data());
    }

    String::from("Cannot render primitive values in a table. Try using the Raw Messages panel instead.")
  }

  fn get_data(&self) -> Vec<&CellValue> {
    if is_typed_array(&self.value) {
      vec![&self.value]
    } else {
      self.value.downcast_ref::<HashMap<String, CellValue>>().unwrap_or(&HashMap::new())
        .iter()
        .map(|(_, value)| value)
        .collect()
    }
  }

  fn render_table(columns: Vec<MergedColumnsType>, data: Vec<&CellValue>) -> String {
    let table = TableComponent {
      columns,
      data,
    };

    table.render()
  }
}

struct TableComponent {
  columns: Vec<MergedColumnsType>,
  data: Vec<&CellValue>,
}

impl TableComponent {
  fn render(&self) -> String {
    let header_rows = self.columns
      .iter()
      .map(|column| {
        row::HeaderCell {
          accessor_path: column.accessor.clone(),
          label: column.label.to_string(),
        }
      })
      .collect();

    let body_rows = self.data.iter().map(|data| {
      row::BodyCell {
        accessor_path: "",
        value: data.to_string(),
      }
    });

    format!(
      r#"<table class="table">
  <thead>
    {}
  </thead>
  <tbody>
    {}
  </tbody>
</table>"#,
      header_rows.join("\n"),
      body_rows.join("\n")
    )
  }
}

struct Row {
  accessor_path: String,
  value: CellValue,
}

struct HeaderCell {
  accessor_path: String,
  label: String,
}

struct BodyCell {
  accessor_path: String,
  value: String,
}
```