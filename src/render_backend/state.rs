use std::sync::Arc;
use std::time::Duration;
use winit::window::Window;

use crate::render_backend::context::WgpuContext;
use crate::render_backend::mesh::Mesh;
use crate::render_backend::scene::{Scene, SceneObject};
use crate::render_backend::instance::{Instance, InstanceBuffer};
use crate::engine::Engine;

pub struct State {
    pub window: Arc<Window>,
    context: WgpuContext,
    render_pipeline: wgpu::RenderPipeline,
    scene: Scene,
    pub engine: Engine,
}

const QUAD_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let mut context = WgpuContext::new(window.clone()).await?;
        let size = window.inner_size();
        context.resize(size.width, size.height);

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
                    buffers: &[
                        crate::render_backend::buffer::Vertex::desc(),
                        InstanceBuffer::vertex_buffer_layout(),
                    ],
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
        let engine = Engine::new();

        // ✅ CRÉER MESHES DEPUIS COLLIDERS

        // Mesh des raquettes (2 instances)
        let paddle_vertices = engine.physics.scene.player1.collider.to_vertices();
        let paddle_mesh = Mesh::from_vertices(&context.device, &paddle_vertices, &QUAD_INDICES);
        let paddle_instances = vec![
            Instance::new(engine.physics.scene.player1.position),
            Instance::new(engine.physics.scene.player2.position),
        ];
        let paddle_buffer = InstanceBuffer::new(&context.device, paddle_instances);
        scene.add_object(SceneObject::new(paddle_mesh, paddle_buffer));

        // Mesh de la balle (1 instance)
        let ball_vertices = engine.physics.scene.ball.collider.to_vertices();
        let ball_mesh = Mesh::from_vertices(&context.device, &ball_vertices, &QUAD_INDICES);
        let ball_instances = vec![Instance::new(engine.physics.scene.ball.position)];
        let ball_buffer = InstanceBuffer::new(&context.device, ball_instances);
        scene.add_object(SceneObject::new(ball_mesh, ball_buffer));

        Ok(Self {
            window,
            context,
            render_pipeline,
            scene,
            engine,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
    }

    pub fn update(&mut self, _dt: Duration) {
        self.engine.update();

        // ✅ SYNC POSITIONS : Engine → Renderer

        // Raquettes (objet 0)
        if let Some(paddles) = self.scene.objects_mut().get_mut(0) {
            paddles
                .instance_buffer_mut()
                .update_instance(0, self.engine.physics.scene.player1.position);
            paddles
                .instance_buffer_mut()
                .update_instance(1, self.engine.physics.scene.player2.position);
            paddles.instance_buffer_mut().update(&self.context.queue);
        }

        // Balle (objet 1)
        if let Some(ball) = self.scene.objects_mut().get_mut(1) {
            ball.instance_buffer_mut()
                .update_instance(0, self.engine.physics.scene.ball.position);
            ball.instance_buffer_mut().update(&self.context.queue);
        }
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

            for object in self.scene.objects() {
                render_pass.set_vertex_buffer(0, object.mesh().vertex_buffer().slice(..));
                render_pass.set_vertex_buffer(1, object.instance_buffer().buffer().slice(..));
                render_pass.set_index_buffer(
                    object.mesh().index_buffer().slice(..),
                    wgpu::IndexFormat::Uint16,
                );

                render_pass.draw_indexed(
                    0..object.mesh().num_indices(),
                    0,
                    0..object.instance_buffer().len() as u32,
                );
            }
        }

        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}