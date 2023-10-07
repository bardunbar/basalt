use cgmath::SquareMatrix;
use log::info;
use wgpu::util::DeviceExt;

use crate::texture;

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

struct CameraData {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,

    aspect: f32,
    fov_y: f32,
    z_near: f32,
    z_far: f32,
}

impl CameraData {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fov_y), self.aspect, self.z_near, self.z_far);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self { view_position: [0.0; 4], view_proj: cgmath::Matrix4::identity().into() }
    }

    fn update_view_proj(&mut self, camera_data: &CameraData) {
        self.view_position = camera_data.eye.to_homogeneous().into();
        self.view_proj = camera_data.build_view_projection_matrix().into();
    }
}

pub struct RenderCamera {

    depth_texture: texture::Texture,
    camera_data: CameraData,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    clear_color: wgpu::Color,
    label: String,
}

impl RenderCamera {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, label: &str) -> Self {

        info!("Initializing new Render Camera - {}", label);

        let depth_texture = texture::Texture::create_depth_texture(&device, &config, &format!("{} - depth texture", label));
        let camera_data = CameraData {
            eye: (0.0, 4.0, 8.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fov_y: 45.0,
            z_near: 0.1,
            z_far: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera_data);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} - camera buffer", label)),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some(&format!("{} - camera_bind_group_layout", label)),
            }
        );

        let camera_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }
                ],
                label: Some(&format!("{} - camera_bind_group", label)),
            }
        );

        info!("Render Camera initialization successful - {}", label);

        Self {
            depth_texture,
            camera_data,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            clear_color: wgpu::Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 },
            label: label.to_owned(),
        }
    }

    #[inline]
    pub fn get_depth_view(&self) -> &wgpu::TextureView {
        &self.depth_texture.view
    }

    pub fn resize(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        self.depth_texture = texture::Texture::create_depth_texture(device, config, &format!("{} - depth_texture", self.label));
    }
}