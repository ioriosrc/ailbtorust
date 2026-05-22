```rust
// SPDX-FileCopyrightText: Copyright (C) 2023-2026 Bayerische Motoren Werke Aktiengesellschaft (BMW AG)<lichtblick@bmwgroup.com>
// SPDX-License-Identifier: MPL-2.0

/** Want to render after all other objects in the scene so that they are not occluded by other objects */
const ANNOTATION_FRONT_POSITION = 100000;

/** Render order for given annotations. Higher numbers rendered after lower numbers */
pub const ANNOTATION_RENDER_ORDER: u32 = {
    FILL: 1 + ANNOTATION_FRONT_POSITION,
    LINE_PREPASS: 2 + ANNOTATION_FRONT_POSITION,
    LINE: 3 + ANNOTATION_FRONT_POSITION,
    POINTS: 4 + ANNOTATION_FRONT_POSITION,
    TEXT: 5 + ANNOTATION_FRONT_POSITION,
};

/** we want annotations to show on top of the entire scene. These are material props to achieve that */
pub const annotation_render_order_material_props: &'static str = "
{
    \"transparent\": true,
    \"depthWrite\": false,
    \"depthTest\": false
}";
```