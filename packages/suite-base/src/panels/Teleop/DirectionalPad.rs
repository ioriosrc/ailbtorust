```rust
use std::rc::Rc;

use crate::{DirectionalPadAction, DirectionalPadProps};

use lichtblick_bmw::base::components::Stack;
use lichtblick_bmw::base::panels::Teleop::constants::{svg_paths_disabled, svg_paths_enabled};
use lichtblick_bmw::base::panels::Teleop::types::{DirectionalPadAction, DirectionalPadProps};
use lichtblick_bmw::common::classes;

pub fn directional_pad(props: &DirectionalPadProps) -> Rc<crate::prelude::Node> {
    let { on_action, disabled = false } = props;
    let (current_action, set_current_action) = std::rc::Rc::new(std::cell::RefCell::new(None));

    let mut classes = classes!();
    let cx = classes.clone();

    let handle_mouse_down = Rc::new(move |action: DirectionalPadAction| {
        set_current_action(action);
        on_action(Some(action));
    });

    let handle_mouse_up = Rc::new(|| {
        if current_action.borrow().is_none() {
            return;
        }
        set_current_action(None);
        on_action(None);
    });

    let make_mouse_handlers = move |action: DirectionalPadAction| -> Option<Rc<crate::prelude::Node>> {
        disabled
            .then(|| {
                None
            })
            .unwrap_or_else(|| {
                Some({
                    let handle_mouse_down_clone = Rc::clone(&handle_mouse_down);
                    let handle_mouse_up_clone = Rc::clone(&handle_mouse_up);

                    let node = crate::prelude::Node::builder()
                        .children([
                            crate::prelude::NodeBuilder::new().child(
                                crate::prelude::NodeBuilder::svg()
                                    .style("user-select: none;")
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))
                                                .d(svg_paths_disabled.get(action).unwrap())
                                                .build(),
                                        ])
                                    .append(crate::prelude::NodeBuilder::g())
                                        .children([
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { active: current_action.borrow().unwrap() == action, disabled }))
                                                .d(svg_paths_enabled.get(action).unwrap())
                                                .build(),
                                            crate::prelude::NodeBuilder::path()
                                                .class(classes!(&cx, { disabled }))