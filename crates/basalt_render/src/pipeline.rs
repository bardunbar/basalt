use basalt_resource;

// Render Pipeline needs the following information...
// Bind Group Layouts
//      Camera
//      Texture
//      Light*
// Vertex Buffer Layout
// Shader

// pub fn create_pipeline_from_shader(shader_path: &str, label: &str ) -> wgpu::RenderPipeline {

//     let shader_text = basalt_resource::load_string(shader_path).unwrap();
//     let shader_descriptor = wgpu::ShaderModuleDescriptor {
//         label: Some(label),
//         source: wgpu::ShaderSource::Wgsl(shader_text.into()),
//     };



// }