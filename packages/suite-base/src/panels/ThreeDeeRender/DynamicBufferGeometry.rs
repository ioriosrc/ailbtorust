```rust
use three::{BufferGeometry, BufferAttribute, TypedArray, Usage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DynamicBufferGeometry {
    attributes: HashMap<String, BufferAttribute>,
    usage: Usage,
    item_capacity: usize,
}

impl DynamicBufferGeometry {
    pub fn new(usage: Usage) -> Self {
        Self {
            attributes: HashMap::new(),
            usage,
            item_capacity: 0,
        }
    }

    pub fn set_usage(&mut self, usage: Usage) {
        self.usage = usage;
        for (_, attribute) in self.attributes.iter_mut() {
            attribute.set_usage(usage);
        }
    }

    pub fn create_attribute<T, C>(
        &mut self,
        name: String,
        array_constructor: C,
        item_size: usize,
        // eslint-disable-next-line @lichtblick/no-boolean-parameters
        normalized: bool,
    ) -> Result<BufferAttribute, &'static str> {
        let data = Box::new([0.0; self.item_capacity * item_size]) as Box<[f32]>;
        let attribute = BufferAttribute::new(data.into(), item_size, normalized);
        attribute.set_usage(self.usage);
        self.attributes.insert(name, attribute);
        Ok(attribute)
    }

    pub fn resize(&mut self, itemCount: usize) {
        if itemCount <= self.item_capacity {
            return;
        }

        for (_, attribute) in self.attributes.iter_mut() {
            let data_constructor = *self
                .attributes
                .get(&attribute.name)
                .ok_or("Missing data constructor for attribute")?;
            let data = Box::new([0.0; itemCount * item_size]) as Box<[f32]>;
            let new_attribute = BufferAttribute::new(data.into(), item_size, attribute.normalized);
            new_attribute.set_usage(self.usage);
            self.attributes.insert(attribute.name.clone(), new_attribute);
        }

        self.item_capacity = itemCount;
    }
}
```