use std::sync::Arc;

use snake_engine::EngineContext;
use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::{ActiveEventLoop, EventLoop},
    platform::
        wayland::WindowAttributesExtWayland
    ,
    window::Window,
};

const OBJECTS_COUNT: u32 = 5;

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = |e: &ActiveEventLoop| {
        Arc::new(
                e
                .create_window(
                    Window::default_attributes()
                        .with_title("snake")
                        .with_name("snake-engine", "snake-engine")
                        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
                        .with_min_inner_size(Size::Physical(PhysicalSize { width: 640, height: 480 })).with_max_inner_size(e.available_monitors().next().unwrap().size()),
                )
                .unwrap(),
        )

    };
    let redraw_closure = || {};

    let mut app = EngineContext::new(&event_loop, |event| window(event), redraw_closure);
    event_loop.run_app(&mut app)
}
