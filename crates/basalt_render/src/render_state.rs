use winit::window::Window;
use log::info;

use crate::{camera, model};

pub struct RenderState {

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,

    render_camera: camera::RenderCamera,

    // TEMP
    test_model: model::Model,
}

impl RenderState {

    pub async fn new(window: Window) -> Self {

        let size = window.inner_size();

        // This is our GPU instance, used to create surfaces and adapters
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), dx12_shader_compiler: Default::default() });

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

        RenderState {
            surface,
            device,
            queue,
            config,
            size,
            window,

            render_camera,

            test_model,
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