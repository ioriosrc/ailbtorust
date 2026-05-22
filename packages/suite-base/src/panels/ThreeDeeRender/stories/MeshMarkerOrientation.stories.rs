```rust
use std::time::{Duration, Instant};

use rocket::State;

use crate::{
    models::MarkerMessage,
    services::FixtureService,
    structs::ThreeDeePanelConfig,
    utils::camera_state::DEFAULT_CAMERA_STATE,
};

#[derive(State)]
pub struct MeshMarkerOrientation {
    fixture_service: FixtureService,
}

impl MeshMarkerOrientation {
    pub async fn get_marker_fixture(&self) -> MarkerMessage {
        self.fixture_service.get_marker_fixture().await
    }
}

// Define your routes and handlers here
```