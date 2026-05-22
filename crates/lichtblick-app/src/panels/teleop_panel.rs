// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)
// SPDX-License-Identifier: MPL-2.0

//! Teleop Panel - sends velocity commands to robots.
//! Supports geometry_msgs/msg/Twist and TwistStamped.
//! Provides keyboard and on-screen joystick control.

use leptos::prelude::*;

/// Published Twist command values.
#[derive(Clone, Copy, Debug, Default)]
pub struct TwistCommand {
    pub linear_x: f64,
    pub linear_y: f64,
    pub linear_z: f64,
    pub angular_x: f64,
    pub angular_y: f64,
    pub angular_z: f64,
}

impl TwistCommand {
    pub fn is_zero(&self) -> bool {
        self.linear_x.abs() < 1e-6
            && self.linear_y.abs() < 1e-6
            && self.linear_z.abs() < 1e-6
            && self.angular_x.abs() < 1e-6
            && self.angular_y.abs() < 1e-6
            && self.angular_z.abs() < 1e-6
    }
}

/// Teleop panel component.
#[component]
pub fn TeleopPanel() -> impl IntoView {
    let linear_speed = RwSignal::new(1.0f64);
    let angular_speed = RwSignal::new(1.0f64);
    let current_cmd = RwSignal::new(TwistCommand::default());
    let is_active = RwSignal::new(false);
    let publish_rate_hz = RwSignal::new(10u32);

    // Keyboard state
    let key_forward = RwSignal::new(false);
    let key_backward = RwSignal::new(false);
    let key_left = RwSignal::new(false);
    let key_right = RwSignal::new(false);
    let key_up = RwSignal::new(false);
    let key_down = RwSignal::new(false);

    // Update command based on key state
    let update_cmd = move || {
        let lin = linear_speed.get_untracked();
        let ang = angular_speed.get_untracked();

        let mut cmd = TwistCommand::default();

        if key_forward.get_untracked() {
            cmd.linear_x += lin;
        }
        if key_backward.get_untracked() {
            cmd.linear_x -= lin;
        }
        if key_left.get_untracked() {
            cmd.angular_z += ang;
        }
        if key_right.get_untracked() {
            cmd.angular_z -= ang;
        }
        if key_up.get_untracked() {
            cmd.linear_z += lin;
        }
        if key_down.get_untracked() {
            cmd.linear_z -= lin;
        }

        current_cmd.set(cmd);
        is_active.set(!cmd.is_zero());
    };

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        let handled = match ev.key().as_str() {
            "w" | "W" | "ArrowUp" => {
                key_forward.set(true);
                true
            }
            "s" | "S" | "ArrowDown" => {
                key_backward.set(true);
                true
            }
            "a" | "A" | "ArrowLeft" => {
                key_left.set(true);
                true
            }
            "d" | "D" | "ArrowRight" => {
                key_right.set(true);
                true
            }
            "q" | "Q" => {
                key_up.set(true);
                true
            }
            "e" | "E" => {
                key_down.set(true);
                true
            }
            _ => false,
        };
        if handled {
            ev.prevent_default();
            update_cmd();
        }
    };

    let on_keyup = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "w" | "W" | "ArrowUp" => key_forward.set(false),
            "s" | "S" | "ArrowDown" => key_backward.set(false),
            "a" | "A" | "ArrowLeft" => key_left.set(false),
            "d" | "D" | "ArrowRight" => key_right.set(false),
            "q" | "Q" => key_up.set(false),
            "e" | "E" => key_down.set(false),
            _ => {}
        }
        update_cmd();
    };

    view! {
        <div
            class="panel-container panel-teleop"
            tabindex="0"
            on:keydown=on_keydown
            on:keyup=on_keyup
        >
            <div class="panel-toolbar">
                <span class="panel-title">{"Teleop"}</span>
                <span class="panel-subtitle">{move || {
                    if is_active.get() { "ACTIVE" } else { "Idle" }
                }}</span>
            </div>
            <div class="panel-content teleop-content">
                <div class="teleop-controls">
                    // Direction buttons (visual keyboard)
                    <div class="teleop-dpad">
                        <div class="teleop-row">
                            <button class="teleop-btn" class:active=move || key_up.get()>{"Q ↑"}</button>
                            <button class="teleop-btn" class:active=move || key_forward.get()>{"W ↑"}</button>
                            <button class="teleop-btn" class:active=move || key_down.get()>{"E ↓"}</button>
                        </div>
                        <div class="teleop-row">
                            <button class="teleop-btn" class:active=move || key_left.get()>{"A ←"}</button>
                            <button class="teleop-btn" class:active=move || key_backward.get()>{"S ↓"}</button>
                            <button class="teleop-btn" class:active=move || key_right.get()>{"D →"}</button>
                        </div>
                    </div>

                    // Speed controls
                    <div class="teleop-speeds">
                        <div class="speed-control">
                            <label>{"Linear: "}</label>
                            <input
                                type="range"
                                min="0.1"
                                max="5.0"
                                step="0.1"
                                prop:value=move || format!("{:.1}", linear_speed.get())
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        linear_speed.set(v);
                                    }
                                }
                            />
                            <span>{move || format!("{:.1} m/s", linear_speed.get())}</span>
                        </div>
                        <div class="speed-control">
                            <label>{"Angular: "}</label>
                            <input
                                type="range"
                                min="0.1"
                                max="3.0"
                                step="0.1"
                                prop:value=move || format!("{:.1}", angular_speed.get())
                                on:input=move |ev| {
                                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                        angular_speed.set(v);
                                    }
                                }
                            />
                            <span>{move || format!("{:.1} rad/s", angular_speed.get())}</span>
                        </div>
                    </div>

                    // Current command display
                    <div class="teleop-status">
                        <div class="cmd-display">
                            {move || {
                                let cmd = current_cmd.get();
                                format!(
                                    "lin: ({:.2}, {:.2}, {:.2}) | ang: ({:.2}, {:.2}, {:.2})",
                                    cmd.linear_x, cmd.linear_y, cmd.linear_z,
                                    cmd.angular_x, cmd.angular_y, cmd.angular_z
                                )
                            }}
                        </div>
                        <div class="cmd-hint">
                            {"Click panel & use W/A/S/D + Q/E to control"}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
