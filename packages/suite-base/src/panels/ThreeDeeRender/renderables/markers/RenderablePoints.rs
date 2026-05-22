```rust
use three::{BufferAttribute, DynamicBufferGeometry, Points, PointsMaterial, Vector3};
use std::marker::PhantomData;

pub struct RenderablePoints {
    geometry: DynamicBufferGeometry,
    points: Points<DynamicBufferGeometry, PointsMaterial>,
}

impl RenderablePoints {
    pub fn new(
        topic: &str,
        marker: &Marker,
        receive_time: Option<i64>,
        renderer: &Renderer,
    ) -> Self {
        let mut geometry = DynamicBufferGeometry::new();
        geometry.create_attribute("position", vec![3.0; 3 * marker.points.len()], <f32>::default);
        geometry.create_attribute("color", vec![4.0; 4 * marker.points.len()], <u8>::default);

        let points_material = make_points_material(marker);
        self.points = Points::new(geometry, &points_material);

        self.update(topic, marker, receive_time);
        Self { geometry, points }
    }

    pub fn dispose(&mut self) {
        self.points.material.dispose();
    }

    pub fn update(&mut self, new_marker: &Marker, receive_time: Option<i64>) {
        let prev_marker = self.geometry.attributes.get("position");
        super::update(marker, receive_time, &self.geometry);
        let marker = self.geometry.attributes.get("position");

        let transparent = marker_has_transparency(marker);
        if transparent != marker_has_transparency(prev_marker) {
            self.points.material.transparent = transparent;
            self.points.material.depth_write = !transparent;
            self.points.material.needs_update = true;
        }

        self.points.material.size = marker.scale.x;

        let points_length = marker.points.len();
        self.geometry.resize(points_length);
        self.set_positions(marker, points_length);
        self.set_colors(marker, points_length);
    }

    fn set_positions(&mut self, marker: &Marker, points_length: usize) {
        let attribute = self.geometry.attributes.get("position") as &BufferAttribute;
        let positions = attribute.data.as_slice().to_vec();
        for i in 0..points_length {
            let point = marker.points[i].unwrap();
            positions[3 * i + 0] = point.x as f32;
            positions[3 * i + 1] = point.y as f32;
            positions[3 * i + 2] = point.z as f32;
        }
        attribute.data.clear();
        attribute.data.extend(positions);
        attribute.needs_update = true;
    }

    fn set_colors(&mut self, marker: &Marker, points_length: usize) {
        // Converts color-per-point to a flattened typed array
        let attribute = self.geometry.attributes.get("color") as &BufferAttribute;
        let rgba_data = attribute.data.as_slice().to_vec();
        for i in 0..points_length {
            let color = marker.points[i].unwrap();
            rgba_data[4 * i + 0] = (color[0] * 255) as u8;
            rgba_data[4 * i + 1] = (color[1] * 255) as u8;
            rgba_data[4 * i + 2] = (color[2] * 255) as u8;
            rgba_data[4 * i + 3] = (color[3] * 255) as u8;
        }
        attribute.data.clear();
        attribute.data.extend(rgba_data);
        attribute.needs_update = true;
    }
}
```

Note that the Rust code snippet above is a direct port of the TypeScript/React version. It uses `BufferAttribute` instead of `Float32Array`, and it does not include any dependency on `three.js`. This code should be used within a Rust environment where `three.js` is available or can be included using a package manager like `Cargo`.