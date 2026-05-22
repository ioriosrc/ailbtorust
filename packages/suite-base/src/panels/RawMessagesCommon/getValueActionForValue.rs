```rust
use crate::{MessagePathStructureItem, ValueAction};

fn build_filter_path(
  multi_slice_path: &str,
  path_item: &str,
  value: &dyn std::any::Any,
  structure_item: Option<&MessagePathStructureItem>,
) -> String {
  if multi_slice_path.ends_with("[:]") && structure_item.is_some() && structure_item.unwrap().structure_type == "primitive" {
    return format!("{}{:=}", multi_slice_path, path_item, value);
  }
  "".to_string()
}

fn find_typical_filter_name(structure_item: &Option<&MessagePathStructureItem>) -> Option<&str> {
  if let Some(next_structure_item) = structure_item {
    if next_structure_item.structure_type == "message" {
      for (key, next_next_structure_item) in next_structure_item.next_by_name.iter() {
        if next_next_structure_item.structure_type == "primitive" && is_typical_filter_name(key) {
          return Some(key);
        }
      }
    }
  }
  None
}

fn build_single_slice_path_for_array(
  path_item: usize,
  value: &dyn std::any::Any,
  structure_item: Option<&MessagePathStructureItem>,
) -> String {
  let typical_filter_name = find_typical_filter_name(structure_item);

  if value.is::<Record>() && !value.is::<Vec<()>>() && typical_filter_name.is_some() {
    let diff_object = value.downcast_ref::<Record>().unwrap();
    let filter_value = diff_object.get(typical_filter_name).unwrap();
    return format!("[{:=}=={:=}]", path_item, filter_value);
  }

  format!("[{:}]}", path_item)
}

fn process_object_element(state: &mut ValueAction) {
  if state.value.is::<Record>() {
    let record = state.value.downcast_ref::<Record>().unwrap();
    let next_structure_item = record.next_by_name.get(&state.key).unwrap();
    let next_value = record.get(&state.key).unwrap();

    *state = ValueAction {
      single_slice_path: format!("{},{:=}", state.single_slice_path, state.key),
      multi_slice_path: format!("{},[]", state.multi_slice_path, state.key),
      filter_path: build_filter_path(state.multi_slice_path, &state.key, next_value, next_structure_item),
      value: next_value,
      structure_item: next_structure_item,
    };
  } else {
    panic!("Invalid structureType for value/pathItem.");
  }
}

fn process_array_element(state: &mut ValueAction) {
  if state.value.is::<Vec<_>>() {
    let vec = state.value.downcast_ref::<Vec<()>>::unwrap();
    let next_value = vec.get(state.key as usize).unwrap();
    let next_structure_item = vec.next();

    *state = ValueAction {
      single_slice_path: format!("{},{:=}", state.single_slice_path, state.key),
      multi_slice_path: format!("{}{}", state.multi_slice_path, "[:]"),
      filter_path: "".to_string(),
      value: next_value,
      structure_item: next_structure_item,
    };
  } else {
    panic!("Invalid structureType for value/pathItem.");
  }
}

fn build_value_action(state: &ValueAction) -> ValueAction {
  if state.value.is::<Record>() {
    let record = state.value.downcast_ref::<Record>().unwrap();
    return ValueAction {
      single_slice_path: state.single_slice_path.clone(),
      multi_slice_path: state.multi_slice_path.clone(),
      primitive_type: record.primitive_type,
      filter_path: state.filter_path.clone(),
    };
  } else if state.structure_item.is_some() && state.structure_item.unwrap().structure_type == "primitive" {
    return ValueAction {
      single_slice_path: state.single_slice_path.clone(),
      multi_slice_path: state.multi_slice_path.clone(),
      primitive_type: state.structure_item.unwrap().primitive_type,
      filter_path: state.filter_path.clone(),
    };
  } else {
    panic!("Invalid structureType or value.");
  }
}

fn get_value_action_for_value(
  root_value: &dyn std::any::Any,
  root_structure_item: Option<&MessagePathStructureItem>,
  key_path: &[&str],
) -> ValueAction {
  let mut state = ValueAction {
    single_slice_path: "".to_string(),
    multi_slice_path: "".to_string(),
    filter_path: "".to_string(),
    value: root_value,
    structure_item: root_structure_item,
  };

  for key in key_path.iter() {
    if state.value.is::<Record>() {
      let record = state.value.downcast_ref::<Record>().unwrap();
      if let Some(next_structure_item) = record.next_by_name.get(key) {
        let next_value = record.get(key).unwrap();

        state = ValueAction {
          single_slice_path: format!("{},{:=}", state.single_slice_path, key),
          multi_slice_path: format!("{},[]", state.multi_slice_path, key),
          filter_path: build_filter_path(state.multi_slice_path, key, next_value, next_structure_item),
          value: next_value,
          structure_item: next_structure_item,
        };
      } else {
        panic!("Key not found in record.");
      }
    } else if state.value.is::<Vec<_>>() {
      let vec = state.value.downcast_ref::<Vec<()>>::unwrap();
      if key.parse::<usize>().ok().map(|i| vec.get(i).unwrap()) {
        let next_value = vec.get(key.parse::<usize>().unwrap()).unwrap();

        state = ValueAction {
          single_slice_path: format!("{},{:=}", state.single_slice_path, key),
          multi_slice_path: format!("{}{}", state.multi_slice_path, "[:]"),
          filter_path: "".to_string(),
          value: next_value,
          structure_item: vec.next(),
        };
      } else {
        panic!("Key not found in vector.");
      }
    } else {
      panic!("Invalid structureType or value.");
    }
  }

  build_value_action(&state)
}

fn get_structure_item_for_path(
  root_structure_item: Option<&MessagePathStructureItem>,
  key_path: &[&str],
) -> Option<&MessagePathStructureItem> {
  let mut structure_item = root_structure_item;

  for key in key_path.iter() {
    if structure_item.is_some() && structure_item.unwrap().structure_type == "message" {
      structure_item = structure_item.unwrap().next_by_name.get(key);
    } else if structure_item.is_some() && structure_item.unwrap().structure_type == "array" {
      structure_item = structure_item.unwrap().next;
    } else if structure_item.is_some() && structure_item.unwrap().structure_type == "primitive" {
      return Some(structure_item.unwrap());
    } else {
      panic!("Invalid structureType or value.");
    }
  }

  None
}
```