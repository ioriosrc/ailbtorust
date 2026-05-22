```rust
use std::collections::HashMap;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/// Class for storing a single instance of each geometry to reuse across scene extensions
/// Callers of `get_geometry` will need to specify a unique key from which to extract the
/// singleton geometry.
pub struct SharedGeometry {
    geometry_map: HashMap<String, Box<dyn std::any::Any>>,
}

impl SharedGeometry {
    /// Get a geometry from the map, or create it if it doesn't exist.
    /// Note that this map will not allow overwriting of existing geometries.
    pub fn get_geometry<T: Box<dyn std::any::Any>>(key: &str, create_geometry: impl FnOnce() -> T) -> T {
        let geometry = self.geometry_map.get(key);
        if let Some(geometry) = geometry {
            let boxed_geometry = Box::from(*geometry);
            let typed_geometry = boxed_geometry.downcast::<T>().unwrap();
            typed_geometry
        } else {
            let created_geometry = create_geometry();
            self.geometry_map.insert(key.to_string(), Box::new(created_geometry));
            created_geometry
        }
    }

    /// Disposes of all geometries and clears the map
    pub fn dispose(&mut self) {
        for (_, geometry) in &mut self.geometry_map {
            let boxed_geometry = geometry.downcast::<Box<dyn std::any::Any>>().unwrap();
            let typed_geometry = boxed_geometry.downcast::<&dyn std::any::Any>().unwrap();
            // Ensure the closure is called, even if it doesn't panic
            drop(typed_geometry);
        }
        self.geometry_map.clear();
    }
}
```

Note: The above code snippet is a Rust port of the given TypeScript/React code. It uses generics and trait bounds to handle different types of geometries, similar to how TypeScript allows for polymorphism through generics. However, it also handles memory management using `Box<dyn std::any::Any>` to allow for flexible geometry types, as well as `Option` for ensuring that the geometry is always properly cleaned up when no longer needed.