```rust
use crate::{
    geometry::Point,
    models::{RenderableSphere, RenderableArrow},
    transforms::Rotation,
};
use std::vec3;

pub enum PublishClickType {
    PoseEstimate,
    Pose,
    Point,
}

pub enum PublishClickState {
    Idle,
    PlaceFirstPoint,
    PlaceSecondPoint,
}

impl PublishClickTool {
    pub fn new(renderer: &Renderer) -> Self {
        let sphere = RenderableSphere::new(
            "",
            RenderableArrow::new(PublishClickType::Point, 2.0, 0.25, 0.25),
            None,
            renderer,
        );
        let arrow = RenderableArrow::new(PublishClickType::Point, 0.25, 0.25, 0.25);

        Self {
            sphere: sphere,
            arrow: arrow,
            publish_click_type: PublishClickType::Point,
            state: PublishClickState::Idle,

            point1: None,
            point2: None,
        }
    }

    pub fn dispose(&mut self) {
        self.arrow.dispose();
        self.sphere.dispose();
    }

    pub fn set_publish_click_type(&mut self, type_: PublishClickType) {
        self.publish_click_type = type_;
        self.update_arrow_marker();
        self.dispatch_event("foxglove.publish-type-change");
    }

    pub fn start(&mut self) {
        self.set_state(PublishClickState::PlaceFirstPoint);
    }

    pub fn stop(&mut self) {
        self.set_state(PublishClickState::Idle);
    }

    pub fn update_arrow_marker(&self) {
        if self.publish_click_type == PublishClickType::Point {
            self.arrow.update(
                RenderableArrow::new(self.publish_click_type, 2.0, 0.25, 0.25),
                None,
            );
        } else {
            let point1 = if let Some(point1) = &self.point1 {
                point1.clone()
            } else {
                vec3(0.0, 0.0, 0.0)
            };

            let mut q = Rotation::identity();
            if self.publish_click_type != PublishClickType::Point {
                let p1 = point1.clone();
                let p2 = if let Some(point2) = &self.point2 {
                    point2.clone()
                } else {
                    vec3(0.0, 0.0, 0.0)
                };
                q.set_from_unit_vectors(
                    vec3::X,
                    (p2 - p1).normalize(),
                );
            }

            self.arrow.update(
                RenderableArrow::new(self.publish_click_type, 0.25, 0.25, 0.25),
                Some(q),
            );
        }
    }

    fn set_state(&mut self, state: PublishClickState) {
        self.state = state;
        match state {
            PublishClickState::Idle => {
                self.point1 = None;
                self.point2 = None;
                self.renderer.remove_listener("click", self.handle_click);
                self.renderer.remove_listener("mousemove", self.handle_mouse_move);
                self.dispatch_event("foxglove.publish-end");
            }
            PublishClickState::PlaceFirstPoint => {
                self.renderer.add_listener("click", self.handle_click);
                self.renderer.add_listener("mousemove", self.handle_mouse_move);
                self.dispatch_event("foxglove.publish-start");
            }
            PublishClickState::PlaceSecondPoint => {}
        }

        self.render();
    }

    fn handle_mouse_move(
        &mut self,
        cursor_coords: (f32, f32),
        world_space_cursor_coords: Option<vec3>,
    ) {
        if let Some(world_space_cursor_coords) = world_space_cursor_coords {
            match self.state {
                PublishClickState::Idle => {}
                PublishClickState::PlaceFirstPoint => {
                    if self.point1.is_none() {
                        self.point1 = Some(world_space_cursor_coords);
                    }
                }
                PublishClickState::PlaceSecondPoint => {
                    if let Some(point2) = &self.point2 {
                        let p = self.point1.clone();
                        let q = Rotation::identity();
                        if self.publish_click_type != PublishClickType::Point {
                            let p1 = point1.clone();
                            let p2 = if let Some(point2) = &self.point2 {
                                point2.clone()
                            } else {
                                vec3(0.0, 0.0, 0.0)
                            };
                            q.set_from_unit_vectors(
                                vec3::X,
                                (p2 - p1).normalize(),
                            );
                        }
                        self.arrow.update(
                            RenderableArrow::new(self.publish_click_type, 0.25, 0.25, 0.25),
                            Some(q),
                        );
                    } else {
                        self.point2 = Some(world_space_cursor_coords);
                    }
                }
            }

            self.render();
        }
    }

    fn handle_click(
        &mut self,
        _cursor_coords: (f32, f32),
        world_space_cursor_coords: Option<vec3>,
        _event: MouseEvent,
    ) {
        if let Some(world_space_cursor_coords) = world_space_cursor_coords {
            match self.state {
                PublishClickState::Idle => {}
                PublishClickState::PlaceFirstPoint => {
                    self.point1 = world_space_cursor_coords.clone();
                    if self.publish_click_type == PublishClickType::Point {
                        self.dispatch_event("foxglove.publish-submit", vec!["point".to_string(), world_space_cursor_coords.to_string()]);
                        self.set_state(PublishClickState::Idle);
                    } else {
                        self.set_state(PublishClickState::PlaceSecondPoint);
                    }
                }
                PublishClickState::PlaceSecondPoint => {
                    self.point2 = world_space_cursor_coords.clone();
                    if let Some(point1) = &self.point1 {
                        let p = point1.clone();
                        let q = Rotation::identity();
                        if self.publish_click_type != PublishClickType::Point {
                            let p1 = point1.clone();
                            let p2 = if let Some(point2) = &self.point2 {
                                point2.clone()
                            } else {
                                vec3(0.0, 0.0, 0.0)
                            };
                            q.set_from_unit_vectors(
                                vec3::X,
                                (p2 - p1).normalize(),
                            );
                        }
                        self.dispatch_event("foxglove.publish-submit", vec![
                            "pose".to_string(),
                            format!("{:?}", vec![p.x(), p.y(), p.z()]),
                            format!("{:?}", q.to_euler_angles().as_array()),
                        ]);
                    }
                    self.set_state(PublishClickState::Idle);
                }
            }

            self.render();
        }
    }

    fn render(&mut self) {
        if self.publish_click_type == PublishClickType::Point {
            self.arrow.visible = false;
            if let Some(point1) = &self.point1 {
                self.sphere.visible = true;
                self.sphere.position.copy_from_vec3(*point1);
            } else {
                self.sphere.visible = false;
            }
        } else {
            self.sphere.visible = false;
            if let Some(point1) = &self.point1 {
                self.arrow.visible = true;

                self.arrow.position.copy_from_vec3(*point1);
                if let Some(point2) = &self.point2 {
                    let p = point1.clone();
                    let q = Rotation::identity();
                    if self.publish_click_type != PublishClickType::Point {
                        let p1 = point1.clone();
                        let p2 = if let Some(point2) = &self.point2 {
                            point2.clone()
                        } else {
                            vec3(0.0, 0.0, 0.0)
                        };
                        q.set_from_unit_vectors(
                            vec3::X,
                            (p2 - p1).normalize(),
                        );
                    }
                    self.arrow.update(
                        RenderableArrow::new(self.publish_click_type, 0.25, 0.25, 0.25),
                        Some(q),
                    );
                } else {
                    self.arrow.quaternion.set(0.0, 0.0, 0.0, 1.0);
                }
            }
        }

        self.renderer.queueAnimationFrame();
    }
}
```