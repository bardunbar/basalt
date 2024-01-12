use cgmath::Zero;
use wgpu::util::DeviceExt;
use winit::window::Window;
use log::info;

use crate::{camera, model::{self, Vertex, Instance}, texture};

pub struct RenderState {

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,

    render_camera: camera::RenderCamera,

    // TEMP
    pub test_model: model::Model,
    pub default_pipeline: wgpu::RenderPipeline,
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
}

impl RenderState {

    pub async fn new(window: Window) -> Self {

        let size = window.inner_size();

        // This is our GPU instance, used to create surfaces and adapters
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });

        // Create the platform specific surface to draw to
        let surface = unsafe {
            instance.create_surface(&window)
        }.unwrap();

        // We will use the adapter to get us a device and a queue to handle communication with the GPU
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface) }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None
            },
            None
        ).await.unwrap();

        info!("Device and Queue created successfully.");

        // Get an sRGB format from the surface capabilities, used to setup a surface config
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Means we can render to the texture from a render pass
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        info!("Surface Configuration Ready\n\tFormat: {:?}\n\tPresent Mode: {:?}\n\tAlpha Mode: {:?}", config.format, config.present_mode, config.alpha_mode);

        // Configures the surface and prepares it for rendering
        surface.configure(&device, &config);

        let render_camera = camera::RenderCamera::new(&device, &config, "main_camera");

        // ***
        // @TODO: Replace temp model loading code
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }
            ]
        });
        let test_model = model::Model::from_string(basalt_resource::load_string("basic_hex.obj").unwrap(), &device, &queue, &texture_bind_group_layout).await.unwrap();
        // ***


        // ***
        // @TODO: Replace temp pipeline code

        let default_pipeline = {
            let default_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("default_pipeline_layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    render_camera.get_bind_group_layout(),
                ],
                push_constant_ranges: &[],
            });

            let shader_text = basalt_resource::load_string("default_instanced.wgsl").unwrap();
            let shader_descriptor = wgpu::ShaderModuleDescriptor {
                label: Some("default_shader"),
                source: wgpu::ShaderSource::Wgsl(shader_text.into()),
            };

            let shader = device.create_shader_module(shader_descriptor);

            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("default_render_pipeline"),
                layout: Some(&default_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[
                        model::ModelVertex::desc(),
                        model::InstanceRaw::desc(),
                    ],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })
                    ],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative:false,
                },
                depth_stencil: Some(wgpu::DepthStencilState{
                    format: texture::Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            })
        };
        // ***

        let instances = {
            fn get_world_pos(q: i32, r: i32) -> cgmath::Vector3<f32> {
                const SIZE: f32 = 1.0;
                let sqrt_3 = f32::sqrt(3.0);

                let x = sqrt_3 * q as f32 + (sqrt_3 * 0.5 * r as f32);
                let y = 0.0;
                let z = 1.5 * r as f32;
                cgmath::Vector3 { x, y, z } * SIZE
            }

            let positions = vec![
                (0,0),
                (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1),
                (2, 0), (1, 1), (0, 2), (-1, 2), (-2, 2), (-2, 1), (-2, 0), (-1, -1), (0, -2), (1, -2), (2, -2), (2, -1),
            ];
            positions.iter().map(|p| {
                let mut position = get_world_pos(p.0, p.1);
                position.y = rand::random();

                let rotation = cgmath::Quaternion::zero();
                let color = cgmath::Vector3::new(0.1, 0.2, 1.0);
                Instance {position, rotation, color}
            }).collect::<Vec<_>>()
        };

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("instance_buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        RenderState {
            surface,
            device,
            queue,
            config,
            size,
            window,

            render_camera,

            test_model,
            default_pipeline,
            instances,
            instance_buffer,
        }
    }

    #[inline]
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    #[inline]
    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    #[inline]
    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    #[inline]
    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    #[inline]
    pub fn get_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    #[inline]
    pub fn get_render_camera(&self) -> &camera::RenderCamera {
        &self.render_camera
    }

    #[inline]
    pub fn get_default_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.default_pipeline
    }

    #[inline]
    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.render_camera.resize(&self.device, &self.config);
        }
    }
}
