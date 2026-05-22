```rust
use std::collections::{HashMap, VecDeque};

struct MarkerPool<T> {
    renderables_by_type: HashMap<T, VecDeque<T>>,
}

impl<T> MarkerPool<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            renderables_by_type: HashMap::new(),
        }
    }

    pub fn acquire(&mut self, type: T, topic: String, marker: Marker, receive_time: Option<bigint>) -> RenderableMarker {
        let renderables = self.renderables_by_type.get_mut(&type);
        if let Some(renderables) = renderables {
            if !renderables.is_empty() {
                let renderable = renderables.pop_back().unwrap();
                renderable.set_settings_path(vec!["topics", topic]);
                renderable.set_settings(marker.frame_locked);
                renderable.set_topic(topic);
                renderable.set_name(format!("{}-{}", topic, marker.ns));
                renderable.update(&marker, receive_time);
                return renderable;
            }
        }

        let renderable = T::new(topic, marker, receive_time, renderer);
        self.renderables_by_type.entry(type).or_insert_with(VecDeque::new).push_back(renderable);

        renderable
    }

    pub fn release(&mut self, renderable: &RenderableMarker) {
        if let Some(renderables) = self.renderables_by_type.get_mut(&renderable.marker.type_) {
            renderables.push_back(renderable.clone());
        }
    }

    pub fn dispose(&mut self) {
        for (_, renderables) in self.renderables_by_type.iter_mut() {
            for renderable in renderables.drain(..) {
                renderable.dispose();
            }
        }
        self.renderables_by_type.clear();
    }
}
```