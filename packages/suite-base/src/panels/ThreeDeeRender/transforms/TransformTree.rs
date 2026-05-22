```rust
use std::collections::{HashMap, HashSet};
use super::*;

pub const DEFAULT_MAX_CAPACITY_PER_FRAME: usize = 10_000;

pub enum AddTransformResult {
    NOT_UPDATED,
    UPDATED,
    CYCLE_DETECTED,
}

// Coordinate frames named in [REP-105](https://www.ros.org/reps/rep-0105.html)
const DEFAULT_FRAME_IDS: &[&str] = &["base_link", "odom", "map", "earth"];

#[derive(Debug)]
pub struct TransformTree {
    transform_pool: ObjectPool<Transform>,
    max_storage_time: Duration,
    max_capacity_per_frame: usize,
    frames: HashMap<String, CoordinateFrame>,
    default_root_frame: CoordinateFrame,

    // For internal use only
    _transform_history: Vec<(Timestamp, CoordinateFrame)>,
}

impl TransformTree {
    pub fn new(
        transform_pool: ObjectPool<Transform>,
        max_storage_time: Duration,
        max_capacity_per_frame: usize,
    ) -> Self {
        let default_root_frame = CoordinateFrame::new(
            FallbackFrameId::FALLBACK_FRAME_ID,
            None,
            max_storage_time,
            max_capacity_per_frame,
            transform_pool.clone(),
        );
        let mut frames = HashMap::new();
        frames.insert(FallbackFrameId::FALLBACK_FRAME_ID.to_string(), default_root_frame);

        Self {
            transform_pool,
            max_storage_time,
            max_capacity_per_frame,
            frames,
            default_root_frame,
            _transform_history: Vec::with_capacity(max_capacity_per_frame),
        }
    }

    pub fn add_transform(
        &mut self,
        frame_id: String,
        parent_frame_id: String,
        time: Timestamp,
        transform: Transform,
    ) -> AddTransformResult {
        let updated = !self.has_frame(&frame_id);
        let cycle_detected = false;

        if !updated && self.parent_of(&frame_id, &parent_frame_id).is_none() {
            return AddTransformResult::CYCLE_DETECTED;
        }

        let frame = self.get_or_create_frame(&frame_id);
        let parent_frame = self.frame(&parent_frame_id);

        // If no parent, or a new one, set the parent
        if (parent_frame.is_none()) || (updated && parent_frame.unwrap().id != parent_frame_id) {
            frame.set_parent(parent_frame.map(|p| p.id.to_string()));
            updated = true;
        }

        if !cycle_detected && !self.frame(&frame_id).add_transform(time, transform).is_ok() {
            cycle_detected = true;
        }

        return if cycle_detected {
            AddTransformResult::CYCLE_DETECTED
        } else if updated {
            AddTransformResult::UPDATED
        } else {
            AddTransformResult::NOT_UPDATED
        };
    }

    pub fn remove_transform(&mut self, child_frame_id: String, parent_frame_id: String, stamp: Timestamp) {
        let child = self.frame(&child_frame_id);
        if let Some(child) = child {
            if let Some(parent) = self.frame(&parent_frame_id) {
                child.remove_transform_at(stamp).unwrap();
            } else {
                child.remove_transform_at(stamp).unwrap();
            }
        }

        // Remove empty ancestors
        self.remove_empty_ancestors(&child_frame_id);
    }

    pub fn remove_empty_ancestors(&mut self, frame_id: &str) {
        let mut to_remove = HashSet::new();

        for parent_id in self.descendants(frame_id).collect::<Vec<&str>>() {
            let parent = self.frame(parent_id).unwrap();
            if parent.transforms_size() > 0 || !self.child_of(parent_id, frame_id).is_none() {
                continue;
            }

            to_remove.insert(parent.id);
        }

        for id in to_remove {
            self.frames.remove(id);
        }
    }

    pub fn clear(&mut self) {
        self._transform_history.clear();
        self.frames.clear();
    }

    pub fn clear_after(&mut self, time: Timestamp) {
        for frame in self
            .frames()
            .iter()
            .filter(|(_, frame)| frame.root_at(time).is_some())
        {
            frame.remove_transforms_after(time);
        }
    }

    pub fn has_frame(&self, id: &str) -> bool {
        self.frames.contains_key(id)
    }

    pub fn frame(&mut self, id: &str) -> Option<CoordinateFrame> {
        self.frames.get(id).map(|f| f.clone())
    }

    pub fn get_or_create_frame(&mut self, id: &str) -> CoordinateFrame {
        if !self.frames.contains_key(id) {
            let frame = CoordinateFrame::new(
                FallbackFrameId::FALLBACK_FRAME_ID,
                None,
                self.max_storage_time,
                self.max_capacity_per_frame,
                self.transform_pool.clone(),
            );
            self.frames.insert(id.to_string(), frame);
        }

        self.frames.get(id).unwrap().clone()
    }

    pub fn frames(&mut self) -> HashMap<String, CoordinateFrame> {
        self.frames
    }

    pub fn apply(
        &self,
        output: Pose,
        input: Readonly<Pose>,
        frame_id: String,
        root_frame_id: Option<String>,
        src_frame_id: String,
        dst_time: Timestamp,
        src_time: Timestamp,
        max_delta: Option<Duration>,
    ) -> Result<Pose, TransformError> {
        let frame = self.frame(&frame_id);
        let src_frame = self.frame(&src_frame_id).ok_or(TransformError::FrameNotFound)?;
        let root_frame =
            (root_frame_id.map(|id| self.frame(id).unwrap()).unwrap_or_else(|| frame.root()))?;

        frame.apply(output, input, &root_frame, src_frame, dst_time, src_time, max_delta)
    }

    pub fn frame_list(&mut self) -> Vec<FrameEntry> {
        let mut output: Vec<FrameEntry> = Vec::new();

        // Create a hierarchy of coordinate frames
        let roots_to_counts: HashMap<String, i32> = HashMap::new();
        for frame in self.frames.values() {
            let root = frame.root().unwrap();
            let root_id = root.id.to_string();

            roots_to_counts.insert(root_id.clone(), *roots_to_counts.get(&root_id).unwrap_or(&0) + 1);
        }
        let mut roots_array: Vec<(String, i32)> = roots_to_counts.into_iter().collect();
        roots_array.sort_by(|a, b| b.1.cmp(&a.1));

        // Convert the `roots_array` hierarchy into a flat list of coordinate frames with depth
        let mut nodes: HashMap<String, Node> = HashMap::new();

        for (id, _) in &roots_array {
            let node = Node::new(id);
            nodes.insert(id.clone(), node);
        }

        let mut to_visit = vec![root_id];

        while !to_visit.is_empty() {
            let id = to_visit.pop().unwrap();
            if let Some(node) = nodes.get_mut(&id) {
                for child_id in self.descendants(&id).collect::<Vec<&str>>() {
                    let child_node = Node::new(child_id);
                    nodes.insert(child_id.clone(), child_node);
                }

                node.set_children(nodes.values().map(|n| n.id.to_string()).collect());
            }
        }

        let mut sorted_nodes: Vec<Node> = nodes.into_values().collect();
        sorted_nodes.sort_by(|a, b| a.label.cmp(&b.label));

        for node in sorted_nodes {
            output.push(FrameEntry::new(node));
        }

        output
    }

    pub fn default_follow_frame_id(&self) -> Option<String> {
        let all_frames = self.frames();

        if all_frames.is_empty() {
            return None;
        }

        // Prefer frames from [REP-105](https://www.ros.org/reps/rep-0105.html)
        for frame_id in DEFAULT_FRAME_IDS {
            if let Some(frame) = self.frame(frame_id) {
                return Some(frame.id.to_string());
            }
        }

        // Choose the root frame with the most children
        let roots_to_counts: HashMap<String, usize> = all_frames.iter().map(|(_, frame)| (frame.root(), 1)).collect();
        let mut roots_array: Vec<(String, usize)> = roots_to_counts.into_iter().collect();

        roots_array.sort_by(|a, b| b.1.cmp(&a.1));

        let root_id = roots_array[0].0;

        Some(root_id.to_string())
    }

    fn parent_of(&self, id: &str, child_id: &str) -> Option<String> {
        self.frames
            .get(id)
            .and_then(|frame| frame.parent().map(|p| p.id))
            .filter(|&id| id == child_id)
            .cloned()
    }

    fn child_of(&self, parent_id: &str, child_id: &str) -> bool {
        self.frames
            .get(parent_id)
            .and_then(|frame| frame.children.contains(child_id))
            .unwrap_or(false)
    }

    fn descendants<'a>(&'a self, id: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        let mut stack = vec![id];
        std::iter::from_iter(&mut stack).flat_map(move |current| {
            if let Some(frame) = self.frames.get(current) {
                frame.children.iter().filter(|child_id| !stack.contains(child_id)).collect()
            } else {
                Vec::new()
            }
        })
    }

    fn remove_empty_ancestors(&mut self, frame_id: &str) {
        let mut to_remove = HashSet::new();

        for parent_id in self.descendants(frame_id).collect::<Vec<&str>>() {
            let parent_frame = self.frame(parent_id).unwrap();
            if parent_frame.transforms_size() > 0 || !self.child_of(parent_id, frame_id).is_none() {
                continue;
            }

            to_remove.insert(parent_id);
        }

        for id in to_remove {
            self.frames.remove(id);
        }
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    label: String,
    children: Vec<String>,
}

impl Node {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            label: CoordinateFrame::DisplayName(id.to_string()),
            children: Vec::new(),
        }
    }

    fn set_children(&mut self, children: Vec<String>) {
        self.children = children;
    }
}
```