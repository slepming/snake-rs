use std::error::Error;

use rapier2d::{math::Vec2, prelude::RigidBodyBuilder};
use snake_engine::GameContext;
use winit::event_loop::EventLoop;

const OBJECTS_COUNT: u32 = 5;

fn main() -> Result<(), impl Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut app = GameContext::new(&event_loop);
    for i in 0..OBJECTS_COUNT {
        app.create_drawable_physics(
            Vec2::new(0.1, 0.1),
            Some(Vec2::new(1920.0 / OBJECTS_COUNT as f32 * i as f32, 100.0)),
            None,
        );
    }
    app.create_drawable_physics(
        Vec2::new(1.0, 0.1),
        Some(Vec2::new(810.0, -500.0)),
        Some(RigidBodyBuilder::fixed()),
    );
    event_loop.run_app(&mut app)
}
