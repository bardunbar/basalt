use wgpu::Color;

use crate::{render_state::RenderState, model};


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
            let render_camera = state.get_render_camera();

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 } ),
                        store: true,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: render_camera.get_depth_view(),
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: true }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_vertex_buffer(1, state.instance_buffer.slice(..));

            render_camera.setup_bindings(&mut render_pass);

            render_pass.set_pipeline(state.get_default_pipeline());

            model::draw_model(&mut render_pass, &state.test_model);
        }

        state.get_queue().submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}