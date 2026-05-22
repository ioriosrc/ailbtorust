```rust
use glam::{Vec2, Vec3};
use serde_json::Value;

// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

use std::rc::Rc;
use yew::{html, Component, Props};

#[derive(Properties)]
struct HoverTooltipProps {
    entities: Vec<HoverEntityInfo>,
    position: Vec3<f64>,
}

pub struct HoverTooltip;

impl Component for HoverTooltip {
    type Message = ();
    type Properties = HoverTooltipProps;

    fn create(_ctx: &Context<Self>) -> Self {
        HoverTooltip
    }

    fn update(&mut self, msg: Self::Message) {}

    fn view(&self, ctx: &Context<Self>) -> Html<Self::Properties> {
        let entities = &self.props.entities;
        let position = &self.props.position;

        let hover_tooltip_state = Rc::new(hover_tooltip_state::HoverTooltipState {
            mode: HoverTooltipStateMode::Hidden,
            visible_entities: Vec::new(),
            frozen_position: position.clone(),
            pending_entities: Vec::new(),
        });

        html! {
            <div class="hover-tooltip">
                {if hover_tooltip_state.mode == HoverTooltipStateMode::Following || hover_tooltip_state.mode == HoverTooltipStateMode::Settled || hover_tooltip_state.mode == HoverTooltipStateMode::ClickPinned {
                    let hovered_entity = entities.iter().find(|e| e.topic == hover_tooltip_state.visible_entities[0].topic && e.entity_id == hover_tooltip_state.visible_entities[0].entity_id).unwrap();
                    html! {
                        <div class="tooltip-content">
                            {for hovered_entity.metadata.iter().map(|kv| html! {
                                <div key={kv.key} class="key-value">
                                    <div class="key">{kv.key}</div>
                                    <div class="value">{kv.value}</div>
                                </div>
                            })}
                        </div>
                    }
                } else if hover_tooltip_state.mode == HoverTooltipStateMode::Grace {
                    let hovered_entity = entities.iter().find(|e| e.topic == hover_tooltip_state.pending_entities[0].topic && e.entity_id == hover_tooltip_state.pending_entities[0].entity_id).unwrap();
                    html! {
                        <div class="tooltip-content">
                            {for hovered_entity.metadata.iter().map(|kv| html! {
                                <div key={kv.key} class="key-value">
                                    <div class="key">{kv.key}</div>
                                    <div class="value">{kv.value}</div>
                                </div>
                            })}
                        </div>
                    }
                }}
            </div>
        }
    }
}

enum HoverTooltipStateMode {
    Hidden,
    Following,
    Settled,
    ClickPinned,
    Grace,
}

struct HoverTooltipState {
    mode: HoverTooltipStateMode,
    visible_entities: Vec<HoverEntityInfo>,
    frozen_position: Vec3<f64>,
    pending_entities: Vec<HoverEntityInfo>,
}
```