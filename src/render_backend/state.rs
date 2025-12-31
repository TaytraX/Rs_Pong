use std::sync::Arc;
use std::time::Duration;
use wgpu::util::DeviceExt;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;

use crate::camera::{Camera, CameraController, CameraUniform, Projection};
use crate::renderer::context::WgpuContext;
use crate::renderer::Material;
use crate::renderer::Mesh;
use crate::renderer::InstanceBuffer;
use crate::renderer::RenderPipelineBuilder;
use crate::renderer::{Scene, SceneObject};
use crate::renderer::model::Model;
use crate::texture::Texture;

pub struct State {
    pub window: Arc<Window>,
    context: WgpuContext,
    render_pipeline: wgpu::RenderPipeline,
    camera: Camera,
    projection: Projection,
    pub camera_controller: CameraController,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_uniform: CameraUniform,
    depth_texture: Texture,
    scene: Scene,
    model: Model,
}
// lib.rs
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
    position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding2: u32,
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let mut context = WgpuContext::new(window.clone()).await?;

        let size = window.inner_size();
        context.resize(size.width, size.height);

        // Camera setup
        let camera = Camera::new(
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
        );
        let projection = Projection::new(
            context.config.width,
            context.config.height,
            cgmath::Deg(45.0),
            0.1,
            100.0,
        );
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera, &projection);

        let camera_buffer =
            context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[camera_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let camera_bind_group_layout = Self::create_camera_bind_group_layout(&context.device);

        let camera_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let light_uniform = LightUniform {
            position: [2.0, 2.0, 2.0],
            _padding: 0,
            color: [1.0, 1.0, 1.0],
            _padding2: 0,
        };

        // We'll want to update our lights position, so we use COPY_DST
        let light_buffer = context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Light VB"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let light_bind_group_layout =
            context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let light_bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        // Pipeline
        let pipeline_builder = RenderPipelineBuilder::new(context.device.clone());
        let render_pipeline = pipeline_builder.build(context.format(), &camera_bind_group_layout);

        // Depth texture
        let depth_texture = Texture::create_depth_texture(
            &context.device,
            &context.config,
            Some("Depth Texture"),
        );

        // Load mesh and material
        let mesh = Mesh::from_glb(
            &context.device,
            "C:\\Users\\grego\\Game_Dev\\Mini_Game_1\\renderer\\rendering\\src\\model\\player.glb",
        )?;

        let model = Model::load(&context.device, "C:\\Users\\grego\\Game_Dev\\teste\\src\\models\\rocket.glb").unwrap();

        let material = Material::new(
            &context.device,
            &context.queue,
            include_bytes!("../images/tree.jpeg"),
            "tree",
        )?;

        let render_pipeline_layout = context.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[
                &camera_bind_group_layout,
                &light_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        // Create instances
        let instances = Scene::create_grid_instances(1, 1);
        let instance_buffer = InstanceBuffer::new(&context.device, instances);

        let mut scene = Scene::new();
        scene.add_object(SceneObject::new(mesh, material, instance_buffer));

        Ok(Self {
            window,
            context,
            render_pipeline,
            camera,
            projection,
            camera_controller: CameraController::new(4.0, 0.4),
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            depth_texture,
            scene,
            model
        })
    }

    fn create_camera_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.context.resize(width, height);
        self.projection.resize(width, height);
        self.depth_texture = Texture::create_depth_texture(
            &self.context.device,
            &self.context.config,
            Some("Depth Texture"),
        );
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera, &self.projection);
        self.context.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
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
                            r: 0.75,
                            g: 0.5,
                            b: 0.25,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            // Render all objects in scene
            for object in self.scene.objects() {
                render_pass.set_bind_group(0, object.material().bind_group(), &[]);
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

                self.model.draw(&mut render_pass);
            }
        }

        self.context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}