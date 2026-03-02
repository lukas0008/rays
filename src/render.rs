use crate::{color::Color, vec2::Vec2};
use std::sync::{
    Arc,
    mpsc::{self, Receiver, SyncSender},
};
use wgpu::Trace;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    platform::x11::WindowAttributesExtX11,
    window::{Window, WindowAttributes},
};

pub struct Renderer {
    pub models: Vec<crate::model::Model>,
    pub state: Option<RunningState>,
    pub frame_receiver: Receiver<FrameUpdate>,
    pub frame: Vec<u8>,
    pub init_size: (u32, u32),
    pub on_event: Option<Box<dyn FnMut(winit::event::WindowEvent)>>,
}

pub enum FrameUpdate {
    Full(Vec<u8>),
    Partial { idx: usize, data: Vec<u8> },
    Individual { idx: usize, color: Color },
    Clear(Color),
}

impl Renderer {
    pub fn new(init_size: (u32, u32)) -> (Self, SyncSender<FrameUpdate>) {
        let (sender, frame_receiver) = mpsc::sync_channel(1024);
        (
            Self {
                models: Vec::new(),
                state: None,
                frame_receiver,
                frame: vec![0u8; init_size.0 as usize * init_size.1 as usize * 4],
                init_size,
                on_event: None,
            },
            sender,
        )
    }
    pub fn set_on_event<F: FnMut(winit::event::WindowEvent) + 'static>(&mut self, callback: F) {
        self.on_event = Some(Box::new(callback));
    }
}
pub(crate) struct RunningState {
    pub window: Arc<Window>,
    pub size: PhysicalSize<u32>,
    pub size_f32: Vec2,
    pub scale_factor: f64,
    #[allow(dead_code)]
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub sampler: wgpu::Sampler,
    pub texture: wgpu::Texture,
    pub frame_counter: usize,
    // pub first_paint: bool,
    // pub render_pipeline: wgpu::RenderPipeline,
    // pub vertex_buffer: wgpu::Buffer,
}

impl ApplicationHandler for Renderer {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }
        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_resizable(false)
                    .with_transparent(false)
                    .with_decorations(false)
                    .with_base_size(PhysicalSize::new(self.init_size.0, self.init_size.1)),
            )
            .expect("Failed to create window");
        let window = Arc::new(window);
        window.set_visible(true);
        window.set_transparent(true);

        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web, we'll have to disable some.
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None,
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Trace::default(),
        }))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps
                .alpha_modes
                .iter()
                .cloned()
                .find(|v| *v == wgpu::CompositeAlphaMode::PreMultiplied)
                .unwrap_or(surface_caps.alpha_modes[0]),
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let vshader = device.create_shader_module(wgpu::include_wgsl!("vert.wgsl"));
        let fshader = device.create_shader_module(wgpu::include_wgsl!("frag.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("CPU Framebuffer Texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[
                wgpu::TextureFormat::Rgba8UnormSrgb, // Add the view format here
            ],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            cache: None,
            label: Some("render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vshader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &fshader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format, // your swapchain / surface format
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), // important for transparency
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        self.state = Some(RunningState {
            window: window.clone(),
            size,
            scale_factor,
            size_f32: (size.width as f32, size.height as f32).into(),
            adapter,
            device,
            queue,
            config,
            surface,
            render_pipeline,
            bind_group,
            sampler,
            texture,
            frame_counter: 0,
        });

        window.request_redraw();
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(cb) = &mut self.on_event {
            cb(event.clone());
        }
        let state = if let Some(state) = &mut self.state {
            state
        } else {
            return;
        };
        match event {
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::RedrawRequested => {
                state.frame_counter += 1;
                let frame = state.surface.get_current_texture().unwrap();
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder = state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                let mut texture_dirty = false;
                while let Ok(frame) = self.frame_receiver.try_recv() {
                    match frame {
                        FrameUpdate::Full(data) => self.frame = data,
                        FrameUpdate::Partial { idx, data } => {
                            let start = idx;
                            let end = start + data.len();
                            self.frame[start..end].copy_from_slice(&data);
                        }
                        FrameUpdate::Individual { idx, color } => {
                            let start = idx;
                            self.frame[start..start + 4].copy_from_slice(&color.0.to_be_bytes());
                        }
                        FrameUpdate::Clear(color) => {
                            for pixel in self.frame.chunks_exact_mut(4) {
                                pixel.copy_from_slice(&color.0.to_be_bytes());
                            }
                        }
                    }
                    texture_dirty = true;
                }
                if texture_dirty {
                    state.queue.write_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &state.texture, // the texture you created earlier
                            mip_level: 0,
                            origin: wgpu::Origin3d::ZERO,
                            aspect: wgpu::TextureAspect::All,
                        },
                        &self.frame, // &[u8] CPU buffer
                        wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(4 * state.size.width),
                            rows_per_image: Some(state.size.height),
                        },
                        wgpu::Extent3d {
                            width: state.size.width,
                            height: state.size.height,
                            depth_or_array_layers: 1,
                        },
                    );
                }

                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("render_pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.,
                                    g: 0.,
                                    b: 0.,
                                    a: 0.,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None,
                        })],
                        ..Default::default()
                    });

                    rpass.set_pipeline(&state.render_pipeline);
                    rpass.set_bind_group(0, &state.bind_group, &[]);
                    rpass.draw(0..6, 0..1); // fullscreen quad
                }

                state.queue.submit(Some(encoder.finish()));
                frame.present();
                // TODO
                state.window.request_redraw();
            }
            _ => {}
        }
    }
}
