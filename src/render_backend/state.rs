use std::sync::Arc;
use wgpu::wgt::AccelerationStructureCopy::Clone;
use crate::render_backend::buffer::Vertex;
use winit::window::Window;
use crate::render_backend::context::WgpuContext;
use crate::render_backend::mesh::Mesh;
use crate::render_backend::scene::Scene;

pub struct State {
    pub window: Arc<Window>,
    context: WgpuContext,
    render_pipeline: wgpu::RenderPipeline,
    scene: Scene
}

pub const VERTICES: [Vertex; 8] = [
    // Collider 1: (0.2, 1.0) - Rouge
    Vertex { position: [-0.2, -1.0, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [0.2, -1.0, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [0.2, 1.0, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.2, 1.0, 0.0], color: [1.0, 0.0, 0.0] },

    // Collider 2: (0.1, 0.1) - Vert
    Vertex { position: [-0.1, -0.1, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.1, -0.1, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.1, 0.1, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-0.1, 0.1, 0.0], color: [0.0, 1.0, 0.0] },
];

// Tableau d'indices (2 triangles par collider)
pub const INDICES: [u16; 12] = [
    // Collider 1
    0, 1, 2,
    0, 2, 3,

    // Collider 2
    4, 5, 6,
    4, 6, 7,
];

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let mut context = WgpuContext::new(window.clone()).await?;

        let size = window.inner_size();
        context.resize(size.width, size.height);


        // Pipeline
        let shader = context
            .device
            .create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let render_pipeline = context.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[Vertex::desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: context.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            });

        let mut scene = Scene::new();
        let mesh: Mesh = Mesh::from_vertices(
            &context.device,
            &VERTICES,
            &INDICES,
        );

        scene.add_object(mesh);

        Ok(Self {
            window,
            context,
            render_pipeline,
            scene
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if !self.context.is_configured() {
            return Ok(());
        }

        let output = self.context.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }

        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn update_instance(&mut self, pos: (f32, f32)) {
        if let Some(object) = self.scene.objects_mut().get_mut(0) {
            object.instance_buffer_mut().update_instance(1, pos.into());
            object.instance_buffer_mut().update(&self.context.queue);
        }
    }
}