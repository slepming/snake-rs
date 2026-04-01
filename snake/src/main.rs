use std::error::Error;

use snake_engine::GameContext;
use winit::event_loop::EventLoop;

fn main() -> Result<(), impl Error> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = GameContext::new(&event_loop);

    event_loop.run_app(&mut app)
}
