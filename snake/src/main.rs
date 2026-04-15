use std::error::Error;

use rapier2d::math::Vec2;
use snake_engine::{GameContext, geometry::shapes::Shapes};
use winit::event_loop::EventLoop;

fn main() -> Result<(), impl Error> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = GameContext::new(&event_loop);
    app.create_drawable_physics(
            Some(Vec2::new(0.0, 100.0)),
            Vec2::new(0.1, 0.1),
    );
    app.create_drawable_physics(
            Some(Vec2::new(500.0, 100.0)),
            Vec2::new(0.1, 0.1),
    );
    app.create_drawable(Shapes::Square([0.3, 0.3]), None);
    event_loop.run_app(&mut app)
}
