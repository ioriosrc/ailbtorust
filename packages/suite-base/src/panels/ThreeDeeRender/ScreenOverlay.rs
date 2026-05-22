```rust
use std::f32;

use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::texture::TextureFormat;

struct ScreenOverlay {
    material: Material,
}

impl ScreenOverlay {
    pub fn new(renderer: &mut State) -> Self {
        let geometry = create_geometry();
        let indices = Indices::U32(vec![0, 1, 2]);
        let layout = MeshVertexBufferLayout::new([
            VertexAttribute::with_layout(
                "a_position",
                VertexFormat::Float32x2,
                VertexElementState::default(),
                AttributeUsage::Position,
            ),
        ]);

        let material = Material {
            render_pipeline: renderer.create_render_pipeline_from_shaders(
                Default::default(),
                Default::default(),
                &Self::vert_shader(),
                &Self::frag_shader(),
                PrimitiveTopology::TriangleList,
                1,
                layout,
                Indices::U32(vec![0, 1, 2]),
            ),
        };

        ScreenOverlay { material }
    }

    pub fn set_color(&mut self, color: Color) {
        let mut color_uniform = self.material.render_pipeline.pipeline().uniform_set().get_mut("color").unwrap() as *mut [f32];
        unsafe {
            color_uniform[0] = color.r();
            color_uniform[1] = color.g();
            color_uniform[2] = color.b();
            color_uniform[3] = 1.0;
        }
    }

    fn vert_shader() -> &'static str {
        r#"
            #version 330 core

            layout (location = 0) in vec2 a_position;

            uniform mat4 u_projection_matrix;
            uniform mat4 u_view_matrix;
            uniform vec4 u_color;

            void main() {
                gl_Position = u_projection_matrix * u_view_matrix * vec4(a_position, 1.0, 1.0);
            }
        "#
    }

    fn frag_shader() -> &'static str {
        r#"
            #version 330 core

            uniform vec4 u_color;

            void main() {
                gl_FragColor = u_color;
            }
        "#
    }
}
```