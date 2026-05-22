```rust
use wgpu::{CommandEncoder, RenderBundle, RenderPipelineDescriptor, TextureViewDescriptor};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

struct AppState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    render_bundle: RenderBundle,
    image: Option<wgpu::Texture>,
    size: (u32, u32),
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("AutoSizingCanvas")
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::default();
    let adapter = instance.enumerate_adapters(wgpu::BackendConstraint::PRIMARY).next().unwrap();
    let queue = adapter.request_queue().unwrap();
    let surface = window.make_surface(&instance, &adapter).unwrap();
    let config = surface.get_default_config().unwrap();

    let render_pipeline = create_render_pipeline(
        &queue,
        &adapter,
        &config,
        &[],
        "shaders/shader.vert",
        "shaders/shader.frag",
        true,
    );

    let (size_x, size_y) = config.present_mode.match_mode;
    let mut state = AppState {
        device: adapter.request_device(wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            extensions: wgpu::Extensions::none(),
            limits: wgpu::Limits::default(),
            guid: None,
            queue,
            label: Some("Canvas"),
        })
        .unwrap(),
        queue,
        surface,
        adapter,
        config,
        render_pipeline,
        render_bundle: RenderBundle::new(&state.device, &state.render_pipeline),
        image: None,
        size: (size_x, size_y),
    };

    loop {
        let mut events = event_loop.poll_events();
        for event in events.iter() {
            match event {
                winit::event::Event::WindowEvent { ref event, .. } => {
                    match event {
                        winit::window::WindowEvent::CloseRequested => return,
                        _ => {}
                    }
                }
            }
        }

        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                view: &state.image.as_ref().unwrap().view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.2, g: 0.2, b: 0.3, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&state.render_pipeline);
        state.render_bundle.encode(encoder).unwrap();

        encoder.finish().submit(&state.queue);

        let mut present_future = state.surface.get_current_texture();
        if let Some(texture) = present_future.await {
            let mut view = texture.create_view(&TextureViewDescriptor {
                label: Some("view"),
                format: surface.get_format(),
                usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::RENDER_ATTACHMENT,
            });
            state.render_bundle.copy_to(&view).unwrap();
        }
    }
}

fn create_render_pipeline(
    queue: &wgpu::Queue,
    adapter: &wgpu::Adapter,
    config: &wgpu::SurfaceConfiguration,
    vertex_shader_bytes: &[u8],
    fragment_shader_bytes: &[u8],
    blend_state_enabled: bool,
) -> wgpu::RenderPipeline {
    let shader_module = adapter.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("shader module"),
        source: wgpu::ShaderSource::Bytes(vertex_shader_bytes),
        flags: wgpu::ShaderFlags::empty(),
    });

    let fragment_module = adapter.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("shader module"),
        source: wgpu::ShaderSource::Bytes(fragment_shader_bytes),
        flags: wgpu::ShaderFlags::empty(),
    });

    let bind_group_layout =
        adapter.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupEntry::Texture {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                texture: wgpu::TextureBindingDescriptor {
                    view_type: wgpu::TextureViewType::View,
                    sampler_binding: wgpu::SamplerBindingDescriptor {
                        label: Some("sampler"),
                        comparison_function: None,
                        address_mode_u: wgpu::AddressMode::ClampToEdge,
                        address_mode_v: wgpu::AddressMode::ClampToEdge,
                        address_mode_w: wgpu::AddressMode::ClampToEdge,
                        mipmap_bias: 0.0,
                    },
                },
            }],
        });

    let render_pipeline_layout = adapter.create_render_pipeline_layout(&wgpu::RenderPipelineLayoutDescriptor {
        label: Some("render pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex_buffer = adapter.create_buffer_with_data(
        &wgpu::BufferDescriptor {
            label: Some("vertex buffer"),
            size: 6 * std::mem::size_of::<Vertex>(),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::VERTEX,
            mapped_at_creation: true,
        },
        std::io::Cursor::new(include_bytes!("data/vertex_buffer.bin")),
    );

    let vertex_layout = adapter.create_vertex_layout(&wgpu::VertexLayoutDescriptor {
        steps: &[
            wgpu::VertexStepDescriptor::POSITION(0, 3 * std::mem::size_of::<f32>()),
            wgpu::VertexStepDescriptor::TEXCOORD(1, 2 * std::mem::size_of::<f32>()),
        ],
    });

    let bind_group = adapter.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry::Buffer {
            binding: 0,
            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
            buffer: &vertex_buffer,
        }],
    });

    let render_pipeline = adapter.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render pipeline"),
        layout: &render_pipeline_layout,
        vertex_stage: wgpu::ProgrammableStages {
            vertex: shader_module.as_ref(),
            fragment: Some(fragment_module.as_ref()),
        },
        fragment_state: if blend_state_enabled {
            Some(wgpu::FragmentStateDescriptor {
                label: Some("fragment state"),
                targets: &[Some(wgpu::RenderPassColorAttachmentDescriptor {
                    view: &state.image.as_ref().unwrap().view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.2, g: 0.2, b: 0.3, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                blend: Some(wgpu::BlendDescriptor {
                    color: wgpu::BlendComponent {
                        src: wgpu::BlendFactor::SrcAlpha,
                        dst: wgpu::BlendFactor::OneMinusSrcAlpha,
                    },
                    alpha: wgpu::BlendComponent {
                        src: wgpu::BlendFactor::SrcAlpha,
                        dst: wgpu::BlendFactor::OneMinusSrcAlpha,
                    },
                }),
            })
        } else {
            None
        },
    });

    render_pipeline
}
```

Note that this code is a high-level representation of what the Rust code does and may not directly correspond to the TypeScript/React implementation. The actual implementation would involve more details, such as texture creation, shader compilation, and rendering loop handling.