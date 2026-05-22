```rust
use std::collections::HashMap;

#[derive(Debug)]
pub struct HoverValue {
    value: f64,
    component_id: String,
    type_: &'static str,
}

impl HoverValue {
    fn new(value: f64, component_id: String, type_: &'static str) -> Self {
        HoverValue { value, component_id, type_ }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn get_component_id(&self) -> &str {
        &self.component_id
    }

    pub fn get_type(&self) -> &'static str {
        self.type_
    }
}

#[derive(Debug)]
pub struct HoverValueMap {
    values: HashMap<String, HoverValue>,
}

impl HoverValueMap {
    pub fn new() -> Self {
        HoverValueMap { values: HashMap::new() }
    }

    pub fn add(&mut self, component_id: String, value: f64, type_: &'static str) {
        self.values.insert(component_id, HoverValue::new(value, component_id, type_));
    }

    pub fn get_by_component_id(&self, component_id: &str) -> Option<&HoverValue> {
        self.values.get(component_id)
    }
}

#[derive(Debug)]
pub struct HoverValueStore {
    values_map: HoverValueMap,
}

impl HoverValueStore {
    pub fn new() -> Self {
        HoverValueStore { values_map: HoverValueMap::new() }
    }

    pub fn add(&mut self, component_id: String, value: f64, type_: &'static str) {
        self.values_map.add(component_id, value, type_);
    }

    pub fn get_by_component_id(&self, component_id: &str) -> Option<&HoverValue> {
        self.values_map.get_by_component_id(component_id)
    }
}
```