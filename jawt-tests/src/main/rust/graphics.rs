// Copyright (c) 2025 Gobley Contributors.

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(all(
    target_family = "unix",
    not(target_vendor = "apple"),
    not(target_os = "android")
))]
pub mod unix;

use std::mem;
use std::sync::Mutex;

use futures::executor::block_on;
use wgpu::util::*;
use wgpu::*;

pub trait RenderTarget: Send + Sync {
    fn size(&self) -> (u32, u32);
    unsafe fn create_surface(&self, instance: &Instance) -> Surface<'static>;
}

pub struct RenderContext {
    _target: Box<dyn RenderTarget>,
    surface: Surface<'static>,
    surface_config: Mutex<SurfaceConfiguration>,
    device: Device,
    queue: Queue,
    vertex_buffer: Buffer,
    pipeline: RenderPipeline,
}

impl RenderContext {
    pub fn new(target: impl RenderTarget + 'static) -> RenderContext {
        log::debug!("RenderContext::new()");

        let instance = Instance::new(&InstanceDescriptor::default());
        // Safety: target will be retained by `RenderContext`
        let surface = unsafe { target.create_surface(&instance) };
        let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::LowPower,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("could not retrieve a adapter");

        let (width, height) = target.size();

        log::debug!("RenderContext::new(): width = {width}, height = {height}");
        let surface_config = surface
            .get_default_config(&adapter, width, height)
            .expect("could not retrieve the surface configuration");

        let (device, queue) = block_on(adapter.request_device(&DeviceDescriptor::default(), None))
            .expect("could not retrieve a device");

        surface.configure(&device, &surface_config);
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let shader_module = device.create_shader_module(include_wgsl!("shader.wgsl"));
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            format: wgpu::VertexFormat::Float32x2,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            offset: mem::size_of::<f32>() as u64 * 2,
                            format: wgpu::VertexFormat::Float32x3,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                front_face: FrontFace::Ccw,
                strip_index_format: None,
                cull_mode: None,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });
        RenderContext {
            _target: Box::new(target),
            surface,
            surface_config: Mutex::new(surface_config),
            device,
            queue,
            vertex_buffer,
            pipeline,
        }
    }

    pub fn change_size(&self, width: u32, height: u32) {
        log::debug!("RenderContext::change_size({width}, {height})");

        let mut surface_config = self.surface_config.lock().unwrap();
        if surface_config.width == width || surface_config.height == height {
            return;
        }

        surface_config.width = width;
        surface_config.height = height;
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn render(&self) {
        log::debug!("RenderContext::render()");

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                log::error!("Swap-chain error: {e:?}");
                return;
            }
        };
        let frame_view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(BACKGROUND_COLOR),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..VERTICES.len() as u32, 0..1);
        }

        log::debug!("RenderContext::render(): submit");

        self.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}

static VERTICES: &[Vertex] = &[
    Vertex {
        x: 0.0,
        y: 0.7,
        r: 1.0,
        g: 0.0,
        b: 0.0,
    },
    Vertex {
        x: 0.7,
        y: -0.7,
        r: 0.0,
        g: 1.0,
        b: 0.0,
    },
    Vertex {
        x: -0.7,
        y: -0.7,
        r: 0.0,
        g: 0.0,
        b: 1.0,
    },
];

const BACKGROUND_COLOR: Color = Color {
    r: 0.0157,
    g: 0.1451,
    b: 0.3216,
    a: 1.0,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Vertex {
    x: f32,
    y: f32,
    r: f32,
    g: f32,
    b: f32,
}

unsafe impl bytemuck::Pod for Vertex {}

unsafe impl bytemuck::Zeroable for Vertex {}
