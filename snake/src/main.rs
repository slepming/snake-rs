use std::{error::Error, sync::Arc};

use rapier2d::{math::Vec2, prelude::RigidBodyBuilder};
use snake_engine::{Drawables, EngineContext, Game};
use winit::{
    dpi::{PhysicalSize, Size},
    event_loop::EventLoop,
    platform::wayland::WindowAttributesExtWayland,
    window::Window,
};

const OBJECTS_COUNT: u32 = 5;

fn main() -> Result<(), impl Error> {
    let snake = Snake {};
    let event_loop = EventLoop::new().unwrap();
    let mut app = EngineContext::new(&event_loop, snake);
    event_loop.run_app(&mut app)
}

struct Snake {}

impl Game for Snake {
    fn start(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        drw: &mut Drawables,
    ) -> Arc<Window> {
        for i in 0..OBJECTS_COUNT {
            drw.create_drawable_physics(
                Vec2::new(0.1, 0.1),
                Some(Vec2::new(1920.0 / OBJECTS_COUNT as f32 * i as f32, 100.0)),
                None,
            );
        }
        drw.create_drawable_physics(
            Vec2::new(1.0, 0.1),
            Some(Vec2::new(810.0, -500.0)),
            Some(RigidBodyBuilder::fixed()),
        );
        Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("snake")
                        .with_name("snake-engine", "snake-engine")
                        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
                        .with_min_inner_size(Size::Physical(PhysicalSize {
                            width: 640,
                            height: 480,
                        }))
                        .with_max_inner_size(
                            event_loop.available_monitors().next().unwrap().size(),
                        ),
                )
                .unwrap(),
        )
    }

    fn update(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: &winit::event::WindowEvent,
    ) {
        todo!()
    }
}
