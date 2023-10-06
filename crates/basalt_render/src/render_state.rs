use winit::window::Window;
use log::info;

pub struct RenderState {

    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
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

        RenderState {
            surface,
            device,
            queue,
            config,
            size,
            window
        }
    }

}

impl RenderState {
    pub fn get_window(&self) -> &Window {
        &self.window
    }
}