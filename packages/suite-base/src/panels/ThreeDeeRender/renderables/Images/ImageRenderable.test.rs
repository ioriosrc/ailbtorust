```rust
use std::sync::{Arc, RwLock};

// Define the ImageRenderable struct
pub struct ImageRenderable {
    topic: String,
    renderer: Arc<RwLock<Renderer>>,
    settings: RwLock<ImageUserData>,
}

// Implement the ImageRenderable methods
impl ImageRenderable {
    pub fn new(topic: String, renderer: Arc<RwLock<Renderer>>, settings: ImageUserData) -> Self {
        ImageRenderable {
            topic,
            renderer,
            settings: RwLock::new(settings),
        }
    }

    pub fn set_settings(&self, new_settings: ImageUserData) {
        *self.settings.write().unwrap() = new_settings;
    }

    pub async fn set_image(&mut self, image: &Image) -> Result<(), Box<dyn std::error::Error>> {
        // Implement the logic to decode and store the image
        let decoded_image = decode_image(image.data);
        *self.settings.write().unwrap().image = Some(decoded_image.clone());
        // Assuming decode_image returns a Result with an ImageBitmap or similar
        Ok(())
    }

    pub fn get_decoded_image(&self) -> Option<&ImageBitmap> {
        self.settings.read().unwrap().image.as_ref()
    }

    pub fn dispose(&mut self) {
        // Implement the logic to clean up resources
    }
}

// Define the Image struct
pub struct Image {
    format: String,
    data: Vec<u8>,
    header: ImageHeader,
}

// Define the ImageHeader struct
#[derive(Debug)]
pub struct ImageHeader {
    frame_id: String,
    stamp: Stamp,
}

// Define the Stamp struct
#[derive(Debug)]
pub struct Stamp {
    sec: u32,
    nsec: u32,
}

// Define the Renderer trait
trait Renderer {
    fn queue_animation_frame(&self, callback: impl FnMut(u64) -> ());
    fn normalize_frame_id(&self, id: u64) -> u64;
    fn settings(&self) -> &Settings;
}

// Define the ImageUserData struct
pub struct ImageUserData {
    topic: String,
    settings: Settings,
    first_message_time: u128,
    camera_info: Option<CameraInfo>,
    camera_model: Option<PinholeCameraModel>,
    image: Option<Image>,
    texture: Option<Texture>,
    material: Option<ShaderMaterial>,
    geometry: Option<Geometry>,
    mesh: Option<Mesh>,
    frame_id: String,
    message_time: u128,
    receive_time: u128,
    pose: Pose,
    settings_path: Vec<String>,
}

// Define the CameraInfo struct
#[derive(Debug)]
pub struct CameraInfo {
    width: u32,
    height: u32,
    binning_x: f64,
    binning_y: f64,
    D: Vec<f64>,
    distortion_model: String,
    K: Vec<f64>,
    P: Vec<f64>,
    R: Vec<f64>,
    roi: RoI,
}

// Define the RoI struct
#[derive(Debug)]
pub struct RoI {
    x_offset: u32,
    y_offset: u32,
    height: u32,
    width: u32,
    do_rectify: bool,
}

// Define the PinholeCameraModel struct
#[derive(Debug)]
pub struct PinholeCameraModel {
    // Implement the fields and methods of PinholeCameraModel
}

// Define the Texture struct
struct Texture;

// Define the ShaderMaterial struct
struct ShaderMaterial;

// Define the Geometry struct
struct Geometry;

// Define the Mesh struct
struct Mesh;
```