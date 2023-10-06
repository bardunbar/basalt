use wgpu::Color;

use crate::render_state::RenderState;


pub struct Renderer {

}

impl Renderer {
    pub fn new() -> Self {
        Renderer {  }
    }

    pub fn render(&self, state: &RenderState) -> Result<(), wgpu::SurfaceError>{
        let output = state.get_surface().get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color::BLACK),
                        store: true,
                    }
                })],
                depth_stencil_attachment: None,
            });
        }

        state.get_queue().submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}