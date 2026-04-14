use std::error::Error;

use rapier2d::{math::Vector, prelude::RigidBodyBuilder};
use snake_engine::GameContext;
use winit::event_loop::EventLoop;

fn main() -> Result<(), impl Error> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = GameContext::new(&event_loop);
    app.create_drawable_physics(
            Some(Vector::new(0.0, 100.0)),
            Vector::new(0.1, 0.1),
    );
    app.create_drawable_physics(
            Some(Vector::new(500.0, 100.0)),
            Vector::new(0.1, 0.1),
    );
    event_loop.run_app(&mut app)
}
