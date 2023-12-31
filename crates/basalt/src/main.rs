use log::info;
use winit::{
    event::{ Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode },
    event_loop::{ EventLoop, ControlFlow },
    window::WindowBuilder,
};

use basalt_render::{render_state::RenderState, renderer::Renderer};

async fn run() {

    info!("Basalt Initialization Begin");
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Basalt").build(&event_loop).unwrap();

    let mut state = RenderState::new(window).await;
    let mut renderer = Renderer::new(state.get_device(), state.get_queue(), state.get_config());

    info!("Basalt Loop Begin");

    event_loop.run(move |event, _, control_flow|
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == state.get_window().id() => {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                         input: KeyboardInput{
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                         },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                        renderer.resize(state.get_device(), *physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                        renderer.resize(state.get_device(), **new_inner_size);
                    }
                    _ => {}
                }
            },
            Event::RedrawRequested(window_id) if window_id == state.get_window().id() => {

                // state.update();

                match renderer.render(&state) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                state.get_window().request_redraw();
            },
            _ => {}
        });
}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    pollster::block_on(run());
}
