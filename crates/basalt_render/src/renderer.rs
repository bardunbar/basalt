use wgpu::Color;

use crate::{render_state::RenderState, model};


pub struct Renderer {
    smaa_target: smaa::SmaaTarget
}

impl Renderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, config: &wgpu::SurfaceConfiguration) -> Self {

        let smaa_target = smaa::SmaaTarget::new(
            &device,
            &queue,
            config.width,
            config.height,
            config.format,
            smaa::SmaaMode::Smaa1X
        );

        Renderer {
            smaa_target
        }
    }

    pub fn render(&mut self, state: &RenderState) -> Result<(), wgpu::SurfaceError>{
        let output = state.get_surface().get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let smaa_frame = self.smaa_target.start_frame(state.get_device(), state.get_queue(), &view);

        let mut encoder = state.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let render_camera = state.get_render_camera();

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &smaa_frame,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.4, g: 0.4, b: 0.4, a: 1.0 } ),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: render_camera.get_depth_view(),
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_vertex_buffer(1, state.instance_buffer.slice(..));

            render_camera.setup_bindings(&mut render_pass);

            render_pass.set_pipeline(state.get_default_pipeline());

            model::draw_model_instanced(&mut render_pass, &state.test_model, 0..state.instances.len() as u32);
        }

        state.get_queue().submit(std::iter::once(encoder.finish()));

        smaa_frame.resolve();
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: winit::dpi::PhysicalSize<u32>) {
        self.smaa_target.resize(device, new_size.width, new_size.height);
    }
}