use std::{io::{Cursor, BufReader}, ops::Range};
use wgpu::util::DeviceExt;

use crate::texture;

pub fn draw_mesh<'a>(render_pass: &mut wgpu::RenderPass<'a>, mesh: &'a Mesh, material: &'a Material) {
    draw_mesh_instanced(render_pass, mesh, material, 0..1);
}

pub fn draw_mesh_instanced<'a>(render_pass: &mut wgpu::RenderPass<'a>, mesh: &'a Mesh, material: &'a Material, instances: Range<u32>) {
    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
    render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

    render_pass.set_bind_group(0, &material.bind_group, &[]);

    render_pass.draw_indexed(0..mesh.num_elements, 0, instances);
}

pub fn draw_model<'a>(render_pass: &mut wgpu::RenderPass<'a>, model: &'a Model) {
    draw_model_instanced(render_pass, model, 0..1);
}

pub fn draw_model_instanced<'a>(render_pass: &mut wgpu::RenderPass<'a>, model: &'a Model, instances: Range<u32>) {

    for mesh in &model.meshes {
        let material = &model.materials[mesh.material];
        draw_mesh_instanced(render_pass, mesh, material, instances.clone());
    }
}



pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}


impl Model {
    pub async fn from_string(obj_text: String, device: &wgpu::Device, queue: &wgpu::Queue, layout: &wgpu::BindGroupLayout) -> anyhow::Result<Model> {
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::load_obj_buf_async(
            &mut obj_reader,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
            |p| async move {
                let mat_text = basalt_resource::load_string(&p).unwrap();
                tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
            }).await?;

            let mut materials = Vec::new();
            for m in obj_materials? {
                let diffuse_texture = texture::Texture::from_bytes(device, queue, &basalt_resource::load_binary(&m.diffuse_texture)?, &format!("{}-diffuse", m.name))?;
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view)
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                        }
                    ],
                    label: None,
                });

                materials.push(Material {
                    name: m.name,
                    diffuse_texture,
                    bind_group,
                })
            }

            let meshes = models.into_iter().map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| ModelVertex {
                        position: [
                            m.mesh.positions[i * 3],
                            m.mesh.positions[i * 3 + 1],
                            m.mesh.positions[i * 3 + 2],
                        ],
                        tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                        normal: [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ],
                    })
                    .collect::<Vec<_>>();

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{}-VertexBuffer", &m.name)),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{}-IndexBuffer", &m.name)),
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                Mesh {
                    name: m.name,
                    vertex_buffer,
                    index_buffer,
                    num_elements: m.mesh.indices.len() as u32,
                    material: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

            Ok(Model { meshes, materials })
    }
}

pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
