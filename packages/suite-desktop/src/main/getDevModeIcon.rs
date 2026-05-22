```rust
use std::path::{Path, PathBuf};
use web_sys::Blob;

async fn get_dev_mode_icon() -> Option<Blob> {
    let original_icon = native_image!()
        .create_from_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("src")
                .join("main")
                .join("../../../resources/icon/icon.png"),
        )
        .await;

    if original_icon.is_none() {
        console_error!("Unable to create dev mode icon");
        return None;
    }

    let buffer = original_icon.unwrap().to_bitmap();
    for i in 0..=3 {
        let hsv = tinycolor({ r: buffer[i] as f64, g: buffer[i + 1] as f64, b: buffer[i + 2] as f64 })
            .to_hsv();

        hsv.h = (hsv.h + ROTATION_DEGREES) % 360;
        let rgb = tinycolor({ h: hsv.h, s: hsv.s, v: hsv.v }).to_rgb();

        buffer[i] = rgb.r as u8;
        buffer[i + 1] = rgb.g as u8;
        buffer[i + 2] = rgb.b as u8;
    }

    native_image!()
        .create_from_buffer(buffer.to_vec(), original_icon.unwrap().get_size())
        .await
}
```