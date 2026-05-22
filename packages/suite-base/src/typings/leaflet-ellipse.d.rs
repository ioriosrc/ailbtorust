```rust
use leaflet::LatLngExpression;
use leaflet::PathOptions;

#[derive(Debug)]
pub struct Ellipse {
    latlng: LatLngExpression,
    radius: Radii,
    tilt: f64,
    options: Option<PathOptions>,
}

impl Ellipse {
    pub fn new(latlng: LatLngExpression, radii: Radii, tilt: f64, options: Option<PathOptions>) -> Self {
        Self {
            latlng,
            radius,
            tilt,
            options,
        }
    }

    pub fn get_latlng(&self) -> &LatLngExpression {
        &self.latlng
    }

    pub fn set_latlng(&mut self, latlng: LatLngExpression) {
        self.latlng = latlng;
    }

    pub fn get_tilt(&self) -> f64 {
        self.tilt
    }

    pub fn set_tilt(&mut self, tilt: f64) {
        self.tilt = tilt;
    }

    pub fn get_radius(&self) -> &Radii {
        &self.radius
    }

    pub fn set_radius(&mut self, radius: Radii) {
        self.radius = radius;
    }
}
```