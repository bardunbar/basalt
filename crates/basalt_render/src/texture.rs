use basalt_resource::Resource;
use image::GenericImageView;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {

}

// impl Resource for Texture {
    // fn load(file_name: &str) -> anyhow::Result<Self> {
    //     let data = basalt_resource::load_binary(file_name)?;

    // }
// }