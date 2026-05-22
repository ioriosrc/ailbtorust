```rust
use std::f64;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

extern crate three;

use three::camera::{PerspectiveCamera, Camera};
use three::math::Vector2;
use three::utils::{clamp, sin, cos, tan, PI};

#[derive(Debug)]
pub struct ImageModeCamera {
    model: Option<&dyn ICameraModel>,
    camera_state: DefaultCameraState,
    rotation: i32,
    flip_horizontal: bool,
    flip_vertical: bool,
    aspect_zoom: Vector2,
    canvas_size: Vector2,
    pan_offset: Vector2,
    user_zoom: f64,
}

impl ImageModeCamera {
    pub fn new() -> Self {
        Self {
            model: None,
            camera_state: DefaultCameraState::default(),
            rotation: 0,
            flip_horizontal: false,
            flip_vertical: false,
            aspect_zoom: Vector2::new(1.0, 1.0),
            canvas_size: Vector2::default(),
            pan_offset: Vector2::default(),
            user_zoom: 1.0,
        }
    }

    pub fn update_camera(&mut self, camera_model: Option<&dyn ICameraModel>) {
        self.model = camera_model;
        self.update_projection();
    }

    pub fn set_pan_offset(&mut self, offset: Vector2) {
        self.pan_offset = offset;
        self.update_projection();
    }

    pub fn get_pan_offset(&self, out: &mut Vector2) {
        *out = self.pan_offset;
    }

    pub fn reset_modifications(&mut self) {
        self.pan_offset.set(0.0, 0.0);
        self.user_zoom = 1.0;
        self.update_projection();
    }

    pub fn set_rotation(&mut self, rotation: i32) {
        match rotation {
            0 | 90 | 180 | 270 => self.rotation = rotation,
            _ => self.rotation = 0,
        }
        // By default the camera is facing down the -y axis with -z up,
        // where the image is on the +y axis with +z up.
        // To correct this we rotate the camera 180 degrees around the x axis.
        let euler = Euler::new((PI as f64) * (rotation as f64 / 180.0), 0.0, PI);
        self.quaternion.set_from_euler(euler);

        self.reset_modifications();
    }

    pub fn set_flip_horizontal(&mut self, flip_horizontal: bool) {
        self.flip_horizontal = flip_horizontal;
        self.reset_modifications();
    }

    pub fn set_flip_vertical(&mut self, flip_vertical: bool) {
        self.flip_vertical = flip_vertical;
        self.reset_modifications();
    }

    pub fn update_zoom_from_wheel(&mut self, ratio: f64, cursor_coords: Vector2) {
        let new_zoom = clamp(self.user_zoom * ratio, MIN_USER_ZOOM as f64, MAX_USER_ZOOM as f64);
        let final_ratio = new_zoom / self.user_zoom;
        let half_width = self.canvas_size.x / 2.0;
        let half_height = self.canvas_size.y / 2.0;
        // Adjust pan offset so the zoom is centered around the mouse location
        self.pan_offset.set(
            (half_width + self.pan_offset.x - cursor_coords.x) * final_ratio - half_width + cursor_coords.x,
            (half_height + self.pan_offset.y - cursor_coords.y) * final_ratio - half_height + cursor_coords.y,
        );
        self.user_zoom = new_zoom;
        self.update_projection();
    }

    fn update_projection(&mut self) {
        if let Some(model) = &self.model {
            self.update_aspect_zoom();

            // focal lengths
            let fx = model.fx;
            let fy = model.fy;

            // (cx, cy) image center in pixel coordinates
            // for panning we can take offsets from this in pixel coordinates
            let scale = self.get_effective_scale();
            let flip_pan_x = if self.flip_horizontal { -1.0 } else { 1.0 };
            let flip_pan_y = if self.flip_vertical { -1.0 } else { 1.0 };
            let pan_x, pan_y;
            match self.rotation {
                0 => {
                    pan_x = (self.pan_offset.x * (fx / fy) * flip_pan_x) as f64;
                    pan_y = (self.pan_offset.y * flip_pan_y) as f64;
                }
                90 => {
                    pan_x = (self.pan_offset.y * (fx / fy) * flip_pan_y) as f64;
                    pan_y = -(self.pan_offset.x * flip_pan_x) as f64;
                }
                180 => {
                    pan_x = -(self.pan_offset.x * (fx / fy) * flip_pan_x) as f64;
                    pan_y = -self.pan_offset.y * flip_pan_y;
                }
                270 => {
                    pan_x = -(self.pan_offset.y * (fx / fy) * flip_pan_y) as f64;
                    pan_y = self.pan_offset.x * flip_pan_x;
                }
            }
            let cx = model.cx + pan_x / scale;
            let cy = model.cy + pan_y / scale;

            let near = self.camera_state.near;
            let far = self.camera_state.far;

            // Calculate coordinates of the canvas/viewport edges relative to the center of the camera frame.
            let left: f64, right: f64, top: f64, bottom: f64;
            // Adjustments to center point keep the image centered based on the orientation and fit mode
            let xOffset = ((1.0 / self.aspect_zoom.x - 1.0) * self.canvas_size.x) / 2.0;
            let yOffset = ((1.0 / self.aspect_zoom.y - 1.0) * self.canvas_size.y) / 2.0;
            // These are the original values for rotation == 0:
            let left0 = (-(cx + xOffset) / fx) * near;
            let right0 = ((self.canvas_size.x - cx + xOffset) / fx) * near;
            let top0 = ((cy + yOffset) / fy) * near;
            let bottom0 = (-(self.canvas_size.y - cy + yOffset) / fy) * near;
            match self.rotation {
                0 => {
                    left = left0;
                    right = right0;
                    top = top0;
                    bottom = bottom0;
                }
                90 => {
                    left = bottom0;
                    right = top0;
                    top = -left0;
                    bottom = -right0;
                }
                180 => {
                    left = -right0;
                    right = -left0;
                    top = -bottom0;
                    bottom = -top0;
                }
                270 => {
                    left = -top0;
                    right = -bottom0;
                    top = right0;
                    bottom = left0;
                }
            }

            if self.flip_horizontal {
                let temp = left;
                left = right;
                right = temp;
            }
            if self.flip_vertical {
                let temp = top;
                top = bottom;
                bottom = temp;
            }

            let projection_matrix = PerspectiveCamera::new(
                left, right, top, bottom, near, far,
            );
            self.projection_matrix.copy(&projection_matrix);
            self.projection_matrix_inverse = self.projection_matrix.clone().invert();
        } else {
            // Assuming the camera model is not available
            self.update_projection_matrix();
        }
    }

    fn update_aspect_zoom(&mut self) {
        let model = &self.model;
        if let Some(model) = model {
            // Adapted from https://github.com/ros2/rviz/blob/ee44ccde8a7049073fd1901dd36c1fb69110f726/rviz_default_plugins/src/rviz_default_plugins/displays/camera/camera_display.cpp#L568
            self.aspect_zoom.set(self.user_zoom, self.user_zoom);

            let { width: img_width, height: img_height } = model;

            let fx = model.fx;
            let fy = model.fy;
            let renderer_aspect = self.canvas_size.x as f64 / self.canvas_size.y as f64;
            let image_aspect = img_width as f64 / fx / (img_height as f64 / fy);

            if self.rotation == 90 || self.rotation == 270 {
                renderer_aspect = 1.0 / renderer_aspect;
            }

            if image_aspect > renderer_aspect {
                self.aspect_zoom.y = (self.aspect_zoom.y / image_aspect) * renderer_aspect;
            } else {
                self.aspect_zoom.x = (self.aspect_zoom.x / renderer_aspect) * image_aspect;
            }
        }
    }

    pub fn get_effective_scale(&self) -> f64 {
        clamp(self.user_zoom, MIN_USER_ZOOM as f64, MAX_USER_ZOOM as f64)
    }

    // Assuming the camera model provides these methods
    trait ICameraModel {
        fn fx(&self) -> f64;
        fn fy(&self) -> f64;
        fn cx(&self) -> f64;
        fn cy(&self) -> f64;
    }
}
```