```rust
use std::cmp;

struct CameraInfo {
    header: Header,
    height: usize,
    width: usize,
    distortion_model: String,
    D: Vec<f64>,
    K: Matrix3,
    R: Matrix3,
    P: Matrix3x4,
    binning_x: usize,
    binning_y: usize,
    roi: RegionOfInterest,
}

struct Header {
    stamp: f64,
    frame_id: String,
}

fn project_pixel(out: &mut Vector3, uv: &[f64], camera_model: &CameraModel, settings: &ProjectSettings) -> &mut Vector3 {
    if settings.planar_projection_factor == 0.0 {
        camera_model.project_pixel_to_3d_ray(out, uv);
    } else if settings.planar_projection_factor == 1.0 {
        camera_model.project_pixel_to_3d_plane(out, uv);
    } else {
        let mut temp_vec = Vector3::new(0.0, 0.0, 0.0);
        let mut temp_vec2 = Vector3::new(0.0, 0.0, 0.0);

        camera_model.project_pixel_to_3d_ray(&mut temp_vec, uv);
        camera_model.project_pixel_to_3d_plane(&mut temp_vec2, uv);

        lerp_vec(out, &temp_vec, &temp_vec2, settings.planar_projection_factor);
    }

    multiply_scalar(out, settings.distance);
    out
}

fn normalize_camera_info(message: &PartialMessage<IncomingCameraInfo> & PartialMessage<CameraCalibration>) -> CameraInfo {
    // Handle lowercase field names as well (ROS2 compatibility)
    let D = message.D.take().unwrap_or_default();
    let K = message.K.take().unwrap_or_default();
    let R = message.R.take().unwrap_or_default();
    let P = message.P.take().unwrap_or_default();

    let Dlen = D.len();
    let Klen = K.len();
    let Rlen = R.len();
    let Plen = P.len();

    CameraInfo {
        header: Header {
            stamp: message.timestamp.unwrap_or_else(|| 0.0),
            frame_id: message.frame_id.as_ref().unwrap_or_default().to_string(),
        },
        height: message.height.unwrap_or(0),
        width: message.width.unwrap_or(0),
        distortion_model: message.distortion_model.unwrap_or("".to_string()),
        D: if Dlen > 0 { D.to_vec() } else { Vec::new() },
        K: Matrix3 {
            x: K.clone(),
            y: K.clone(),
            z: K.clone(),
        },
        R: Matrix3 {
            x: R.clone(),
            y: R.clone(),
            z: R.clone(),
        },
        P: Matrix3x4 {
            x: P.clone(),
            y: P.clone(),
            z: P.clone(),
            w: P.clone(),
        },
        binning_x: message.binning_x.unwrap_or(0),
        binning_y: message.binning_y.unwrap_or(0),
        roi: normalize_region_of_interest(message.roi.as_ref().unwrap_or_default()),
    }
}

fn normalize_region_of_interest(roi: &RegionOfInterest) -> RegionOfInterest {
    if let Some(roi) = roi {
        RegionOfInterest {
            x_offset: roi.x_offset.unwrap_or(0),
            y_offset: roi.y_offset.unwrap_or(0),
            height: roi.height.unwrap_or(0),
            width: roi.width.unwrap_or(0),
            do_rectify: roi.do_rectify.unwrap_or(false),
        }
    } else {
        RegionOfInterest::default()
    }
}

fn lerp_vec(out: &mut Vector3, a: &[f64], b: &[f64], t: f64) {
    out.x = (1.0 - t) * a[0] + t * b[0];
    out.y = (1.0 - t) * a[1] + t * b[1];
    out.z = (1.0 - t) * a[2] + t * b[2];
}

fn multiply_scalar(vec: &mut Vector3, scalar: f64) {
    vec.x *= scalar;
    vec.y *= scalar;
    vec.z *= scalar;
}
```