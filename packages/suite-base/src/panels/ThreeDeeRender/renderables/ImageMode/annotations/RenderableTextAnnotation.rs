```rust
use anyhow::{anyhow, Result};
use bevy::prelude::*;
use chrono::Utc;
use colorama::{Color, Style};

const ANNOTATION_RENDER_ORDER: i32 = 0;

pub struct RenderableTextAnnotation {
    label_pool: LabelPool,
    label: Label,

    scale: f32,
    scale_needs_update: bool,

    original_message: Option<RosObject>,
    annotation: Option<NormalizedTextAnnotation>,
    annotation_needs_update: bool,

    camera_model: Option<&ICameraModel>,
    camera_model_needs_update: bool,
}

impl RenderableTextAnnotation {
    pub fn new(topic_name: String, label_pool: LabelPool) -> Result<Self> {
        let label = label_pool.acquire();
        label.mesh.render_order = ANNOTATION_RENDER_ORDER;
        label.set_anchor_point(0.0, 0.0);
        label.set_billboard(true);
        label.setSize_attenuation(false);

        Ok(Self {
            topic_name,
            label_pool,
            label,
            scale: 0.0,
            scale_needs_update: false,
            original_message: None,
            annotation: None,
            annotation_needs_update: false,
            camera_model: None,
            camera_model_needs_update: false,
        })
    }

    pub fn dispose(&mut self) -> Result<()> {
        self.label.mesh.render_order = 0;
        self.label_pool.release(self.label);
        Ok(())
    }

    pub fn details(&self) -> Result<Record<String, RosValue>> {
        if let Some(original_message) = &self.original_message && let Some(annotation) = &self.annotation {
            return Ok({
                annotation: get_annotationAtPath(original_message, annotation.message_path),
                original_message: self.original_message,
            });
        }
        Ok({})
    }

    pub fn set_scale(&mut self, scale: f32) -> Result<()> {
        if scale != self.scale {
            self.scale_needs_update = true;
            self.scale = scale;
        }
        Ok(())
    }

    pub fn set_camera_model(&mut self, camera_model: &ICameraModel) -> Result<()> {
        if self.camera_model != Some(camera_model) {
            self.camera_model_needs_update = true;
            self.camera_model = Some(camera_model);
        }
        Ok(())
    }

    pub fn set_annotation(&mut self, annotation: NormalizedTextAnnotation, original_message: Option<RosObject>) -> Result<()> {
        if self.annotation != Some(annotation) || !self.original_message.is_some() && original_message.is_some() {
            self.annotation_needs_update = true;
            self.original_message = original_message;
            self.annotation = annotation;
        }
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        if let (Some(annotation), Some(camera_model)) = (&self.annotation, &self.camera_model) {
            if self.annotation_needs_update || self.scale_needs_update {
                self.label.set_line_height(annotation.font_size * self.scale);
                self.scale_needs_update = false;
            }

            if self.annotation_needs_update {
                self.label.set_text(&annotation.text);
                self.label.color(
                    annotation.foreground_r,
                    annotation.foreground_g,
                    annotation.foreground_b,
                    annotation.foreground_a,
                );

                if annotation.background_color.is_some() {
                    self.label.background_color(
                        annotation.background_color.r,
                        annotation.background_color.g,
                        annotation.background_color.b,
                        annotation.background_color.a,
                    );
                } else {
                    let foreground_is_dark = get_luminance(annotation.foreground_r, annotation.foreground_g, annotation.foreground_b) < 0.5;
                    if foreground_is_dark {
                        self.label.background_color(1.0, 1.0, 1.0);
                    } else {
                        self.label.background_color(0.0, 0.0, 0.0);
                    }
                }
            }

            if self.annotation_needs_update || self.camera_model_needs_update {
                camera_model.project_pixel_to_3d_plane(&mut self.label.position, annotation.position);
            }

            self.annotation_needs_update = false;
            self.camera_model_needs_update = false;
        }

        Ok(())
    }
}
```