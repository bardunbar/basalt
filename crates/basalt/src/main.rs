use winit::{
    event::{ Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode },
    event_loop::{ EventLoop, ControlFlow },
    window::WindowBuilder,
};


async fn run() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow|
        match event {
            Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                         input: KeyboardInput{
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                         },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    // WindowEvent::Resized(physical_size) => {
                    //     state.resize(*physical_size);
                    // },
                    // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //     state.resize(**new_inner_size);
                    // }
                    _ => {}
                }
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // state.update();
                // match state.render() {
                //     Ok(_) => {}
                //     Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                //     Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                //     Err(e) => eprintln!("{:?}", e),
                // }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            _ => {}
        });
}

fn main() {
    pollster::block_on(run());
}
