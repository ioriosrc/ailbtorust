```rust
use std::rc::Rc;

use color::{Color, Luminance};
use lichtblick::{
    rostime::{Duration, NanoSec},
    scene_entity as SceneEntity,
    scene_settings as LayerSettingsEntity,
    text_primitive as TextPrimitive,
};
use three_text::{Label, LabelPool};

use crate::renderer::{IRenderer, RenderablePrimitive};

pub struct RenderableTexts {
    renderer: Rc<dyn IRenderer>,
    labels: Vec<Label>,
}

impl RenderableTexts {
    pub fn new(renderer: Rc<dyn IRenderer>) -> Self {
        Self {
            renderer,
            labels: Vec::new(),
        }
    }

    fn ensure_capacity(&mut self, new_length: usize) {
        if new_length > self.labels.len() {
            for _ in 0..(new_length - self.labels.len()) {
                let label = self.renderer.label_pool.acquire();
                self.labels.push(label);
                self.add(label);
            }
        }
    }

    fn update_texts(&mut self, texts: &[TextPrimitive]) {
        self.ensure_capacity(texts.len());
        let override_color = if let Some(color) = self.userData.settings.color {
            color.to_rgba()
        } else {
            None
        };

        let mut i = 0;
        for text in texts {
            let color = override_color.or_else(|| text.color.to_rgba());

            let label = self.labels.get(i).unwrap_or_else(|| panic!("invariant: labels array smaller than requested"));

            label.set_text(text.text);
            label.set_color(
                SRGBToLinear(color.red()),
                SRGBToLinear(color.green()),
                SRGBToLinear(color.blue()),
                color.alpha(),
            );

            let foreground_is_dark = get_luminance(color.red(), color.green(), color.blue()) < 0.5;
            if foreground_is_dark {
                label.set_background_color(1, 1, 1, color.alpha());
            } else {
                label.set_background_color(0, 0, 0, color.alpha());
            }
            label.set_line_height(text.font_size);
            // note that billboard needs to be true for scale_invariant to work
            label.set_billboard(text.billboard);
            // attenuation -> size accounts for distance from camera
            // scale_invariant negates this and should make it the same size always
            label.set_size_attenuation(!text.scale_invariant);
            label.set_quaternion(
                text.pose.orientation.x(),
                text.pose.orientation.y(),
                text.pose.orientation.z(),
                text.pose.orientation.w(),
            );

            label.set_position(text.pose.position.x(), text.pose.position.y(), text.pose.position.z());
            i += 1;
        }
        // need to release the no longer used labels so that they don't linger on the scene
        if i < self.labels.len() {
            for label in self.labels.iter().skip(i) {
                self.renderer.label_pool.release(label.clone());
            }
        }
    }

    pub fn dispose(&self) {
        for label in &self.labels {
            self.renderer.label_pool.release(label);
        }
    }

    pub fn update(
        &mut self,
        topic: Option<&str>,
        entity: Option<&SceneEntity>,
        settings: LayerSettingsEntity,
        receive_time: NanoSec,
    ) {
        super::RenderablePrimitive::update(&self, topic, entity, settings, receive_time);
        if let Some(entity) = entity {
            let lifetime_ns = Duration::new(entity.lifetime).as_nanos();
            self.userData.expires_at = lifetime_ns == 0.into() ? None : receive_time + lifetime_ns;
            self.update_texts(entity.texts());
        }
    }

    pub fn update_settings(&mut self, settings: LayerSettingsEntity) {
        self.update(self.userData.topic.clone(), self.userData.entity.clone(), settings, self.userData.receive_time);
    }
}
```