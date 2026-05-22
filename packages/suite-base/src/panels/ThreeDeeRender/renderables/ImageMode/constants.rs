```rust
use std::collections::HashSet;

// Define constants for the image topic paths
pub const IMAGE_TOPIC_PATH: [&str; 2] = ["imageMode", "imageTopic"];
pub const DECODE_IMAGE_ERR_KEY: &str = "CreateBitmap";

pub const IMAGE_MODE_HUD_GROUP_ID: &str = "IMAGE_MODE";

// Define constants for HUD item IDs
pub const BOTH_TOPICS_DO_NOT_EXIST_HUD_ITEM_ID: &str = "BOTH_TOPICS_DO_NOT_EXIST";
pub const IMAGE_TOPIC_DOES_NOT_EXIST_HUD_ITEM_ID: &str = "IMAGE_TOPIC_DOES_NOT_EXIST";
pub const CALIBRATION_TOPIC_DOES_NOT_EXIST_HUD_ITEM_ID: &str = "CALIBRATION_TOPIC_DOES_NOT_EXIST";
pub const WAITING_FOR_SYNC_NOTICE_HUD_ID: &str = "WAITING_FOR_SYNC_NOTICE";
pub const WAITING_FOR_SYNC_EMPTY_HUD_ID: &str = "WAITING_FOR_SYNC_EMPTY";
pub const WAITING_FOR_IMAGES_EMPTY_HUD_ID: &str = "WAITING_FOR_IMAGES_EMPTY";
pub const WAITING_FOR_BOTH_MESSAGES_HUD_ID: &str = "WAITING_FOR_BOTH_MESSAGES";
pub const WAITING_FOR_CALIBRATION_HUD_ID: &str = "WAITING_FOR_CALIBRATION";
pub const WAITING_FOR_IMAGES_NOTICE_ID: &str = "WAITING_FOR_IMAGES_NOTICE";

// Define constants for ROS image and compressed image datatypes
pub const ROS_IMAGE_DATATYPES: HashSet<&str> = [
    "raw", "jpeg", "png", "webp", "avif", "tiff", "jpgx",
];
pub const ROS_COMPRESSED_IMAGE_DATATYPES: HashSet<&str> = [
    "jpg", "png", "webp", "avif", "tiff", "jp2",
];

// Define constants for raw image datatypes
pub const RAW_IMAGE_DATATYPES: HashSet<&str> = [
    "jpeg", "png", "gif", "bmp", "tif", "jpgx", "avif",
];

// Define constants for all supported image schemas
pub const ALL_SUPPORTED_IMAGE_SCHEMAS: HashSet<&str> = [
    &"raw",
    &"jpeg",
    &"png",
    &"webp",
    &"avif",
    &"tiff",
    &"jpgx",
    &"bmp",
    &"gif",
    &"tif",
    &"jp2",
];

// Define constants for supported raw image schemas
pub const SUPPORTED_RAW_IMAGE_SCHEMAS: HashSet<&str> = [
    &"raw",
    &"jpeg",
    &"png",
    &"gif",
    &"bmp",
    &"tif",
    &"jpgx",
    &"avif",
];

// Define constants for all supported calibration schemas
pub const ALL_SUPPORTED_CALIBRATION_SCHEMAS: HashSet<&str> = [
    "camera_info", "camera_calibration",
];

// Define constants for minimum and maximum brightness values
pub const MIN_BRIGHTNESS: f32 = 0.0;
pub const MAX_BRIGHTNESS: f32 = 100.0;
pub const INITIAL_BRIGHTNESS: f32 = (MAX_BRIGHTNESS + MIN_BRIGHTNESS) / 2.0;

// Define constants for minimum and maximum contrast values
pub const MIN_CONTRAST: f32 = 0.0;
pub const MAX_CONTRAST: f32 = 100.0;
pub const INITIAL_CONTRAST: f32 = (MAX_CONTRAST + MIN_CONTRAST) / 2.0;

// Define constants for brightness limits
pub const LOWER_BRIGHTNESS_LIMIT: f32 = -0.6;
pub const UPPER_BRIGHTNESS_LIMIT: f32 = 0.6;

// Define constants for contrast limits
pub const LOWER_CONTRAST_LIMIT: f32 = 0.1;
pub const UPPER_CONTRAST_LIMIT: f32 = 1.9;

// Define constant for no image topics HUD item
pub const NO_IMAGE_TOPICS_HUD_ITEM: HUDItem = {
    id: "NO_IMAGE_TOPICS",
    group: IMAGE_MODE_HUD_GROUP_ID,
    get_message: || t3d("noImageTopicsAvailable"),
    display_type: "empty",
};

// Define constant for default image configuration
pub const DEFAULT_IMAGE_CONFIG: ImageConfig = Default::default();

// Define vertex shader source code
const VERTEX_SHADER: &str = r#"
    varying vec2 vUv;
    void main() {
        vUv = uv;
        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
"#;

// Define fragment shader source code
const FRAGMENT_SHADER: &str = r#"
    uniform sampler2D map;
    uniform float brightness;
    uniform float contrast;
    uniform vec3 color;
    uniform float opacity;
    varying vec2 vUv;

    void main() {
        vec4 texColor = texture2D(map, vUv);

        // Apply brightness
        texColor.rgb += brightness;

        // Apply contrast
        texColor.rgb = ((texColor.rgb - 0.5) * contrast) + 0.5;

        // Apply tint color and opacity
        texColor.rgb *= color;
        texColor.a *= opacity;

        gl_FragColor = texColor;
    }
"#;
```