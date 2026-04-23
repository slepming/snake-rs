use std::sync::Arc;

use rapier2d::{math::Vec2, prelude::RigidBodyBuilder};
use snake_engine::{EngineContext, drw::drawable::Children, mv::transform::PhysicsContext};
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
                        .with_min_inner_size(Size::Physical(PhysicalSize { width: 640, height: 480 }))
                        .with_max_inner_size(e.available_monitors().next().unwrap().size()),
                )
                .unwrap(),
        )

    };

    let redraw_closure = |ch: &mut Children, pc: &mut PhysicsContext| {
        ch.add_physics(pc.create_phys_square(RigidBodyBuilder::dynamic(), 
                [0.1, 0.1], 
                ch.physics_drawables.len() as u32
                    + ch.drawables.len() as u32
                    + 1, 
                Some(Vec2::new(1920.0 / OBJECTS_COUNT as f32 * 1 as f32, 100.0)),
                )
            );
    };

    let mut app = EngineContext::new(&event_loop, |event| window(event), |ch, pc| redraw_closure(ch, pc));
    event_loop.run_app(&mut app)
}
