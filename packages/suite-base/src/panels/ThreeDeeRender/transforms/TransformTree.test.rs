```rust
use std::collections::{HashMap, VecDeque};

struct TransformTree {
    frame_data: HashMap<i64, Option<Transform>>,
}

impl TransformTree {
    fn new(pool: ObjectPool<Transform>) -> Self {
        Self {
            frame_data: HashMap::new(),
        }
    }

    fn add_transform(&mut self, parent_name: &str, child_name: &str, frame_number: i64, transform: Transform) -> AddTransformResult {
        let parent = match self.frame_data.get_mut(parent_name) {
            Some(frame) => frame.as_ref() as *const Transform,
            None => return AddTransformResult::NOT_FOUND,
        };

        let child = match self.frame_data.get_mut(child_name) {
            Some(frame) => frame.as_ref() as *const Transform,
            None => return AddTransformResult::NOT_FOUND,
        };

        if unsafe { (*parent).is_cycle(self, frame_number) } {
            AddTransformResult::CYCLE_DETECTED
        } else if unsafe { (*child).has_cycle(self, frame_number) } {
            AddTransformResult::CYCLE_DETECTED
        } else if unsafe { (*parent).has_child(*child, self, frame_number) } {
            AddTransformResult::UPDATED
        } else {
            let new_frame = TransformTreeFrame::new(transform);
            self.frame_data.insert(frame_number, Some(new_frame));
            AddTransformResult::UPDATED
        }
    }

    fn remove_transform(&mut self, parent_name: &str, child_name: &str, frame_number: i64) -> bool {
        let parent = match self.frame_data.get_mut(parent_name) {
            Some(frame) => frame.as_ref() as *const Transform,
            None => return false,
        };

        let child = match self.frame_data.get_mut(child_name) {
            Some(frame) => frame.as_ref() as *const Transform,
            None => return false,
        };

        if unsafe { (*parent).is_cycle(self, frame_number) } {
            return false;
        }

        if unsafe { (*child).has_child(*parent, self, frame_number) } {
            return false;
        }

        self.frame_data.remove(&frame_number);
        true
    }

    fn frame(&self, name: &str) -> Option<&Transform> {
        match self.frame_data.get(name) {
            Some(frame) => frame.as_ref(),
            None => None,
        }
    }
}

struct TransformTreeFrame {
    transform: Transform,
}

impl TransformTreeFrame {
    fn new(transform: Transform) -> Self {
        Self { transform }
    }

    unsafe fn is_cycle(&self, tree: &TransformTree, frame_number: i64) -> bool {
        if frame_number == 0 {
            return false;
        }

        let parent = self.transform.parent();

        if let Some(parent_name) = parent {
            let parent_frame = tree.frame(parent_name);
            if parent_frame.is_none() || !unsafe { (*parent_frame).is_cycle(tree, frame_number - 1) } {
                return false;
            }
        }

        true
    }

    unsafe fn has_child(&self, child: &Transform, tree: &TransformTree, frame_number: i64) -> bool {
        let children = self.transform.children();

        for child_name in children {
            if !tree.frame(child_name).is_none() && !unsafe { (*tree.frame(child_name)).has_cycle(tree, frame_number - 1) } {
                return true;
            }
        }

        false
    }
}

enum AddTransformResult {
    UPDATED,
    NOT_FOUND,
    CYCLE_DETECTED,
}
```