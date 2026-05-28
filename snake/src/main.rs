use std::{process::exit, sync::Arc};

use color::Rgba8;
use rand::{RngExt, seq::{IndexedRandom, SliceRandom}};
use rapier2d::math::Vec2;
use snake_engine::{
    EngineContext,
    cmd::command::{CommandQueue, DrawCommand},
    drw::drawable::{Children, DrawableCreateInfo},
    geom::shapes::Shapes,
    mv::phys::movement::PhysicsContext,
    res::assets::AssetsManager,
};
use tracing::debug;
use winit::{
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::Window,
};

const OBJECTS_COUNT: u32 = 6;

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut rng_init = rand::rng();

    let mut window = |e: &ActiveEventLoop,
                      ch: &mut Children,
                      assets: &mut AssetsManager,
                      wind: Arc<Window>,
                      command: &mut CommandQueue| {
        let monitor_size = e.available_monitors().next().unwrap().size();
        wind.set_title("snake");
        let mut rng = rng_init.clone();
        for i in 0..OBJECTS_COUNT {
            let (r, g, b) = (rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>());
            let drawable_info = DrawableCreateInfo::default()
                .with_size(Vec2::new(1000.0, 1000.0))
                .with_color(Rgba8 { r, g, b, a: 255 })
                .with_position(Vec2::new(
                    300.0 * i as f32,
                    monitor_size.height as f32 / 2.0,
                ))
                .with_thickness(0.0)
                .with_radius(0.0);

            //dbg!((r, g, b));
            if i % 2 == 0 {
                command.append(DrawCommand::DrawObject(Shapes::Circle, drawable_info));
            } else if i % 3 == 0 {
                command.append(DrawCommand::DrawObject(
                    Shapes::Square,
                    drawable_info.with_size(Vec2::new(50.0, 50.0)),
                ));
            } else {
                command.append(DrawCommand::DrawObject(
                    Shapes::Image(assets.load(std::path::Path::new("image.png"), true)),
                    drawable_info.with_size(Vec2::new(50.0, 50.0)),
                ));
            }
        }
    };

    let redraw_closure = |_ch: &mut Children,
                          _pc: &mut PhysicsContext,
                          assets: &mut AssetsManager,
                          event: &WindowEvent,
                          command: &mut CommandQueue| {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(NamedKey::Escape) => exit(0),
                        Key::Named(NamedKey::Enter) => {
                            let mut shapes: Vec<Shapes> = Vec::with_capacity(2);
                            shapes.extend(vec![Shapes::Circle, Shapes::Square, Shapes::Image(assets.load(std::path::Path::new("image.png"), true))]);
                            let variant = shapes.choose(&mut rng_init.clone()).unwrap();
                            command.append(DrawCommand::DrawObject(
                                variant.clone(),
                                DrawableCreateInfo::default()
                                    .with_position(Vec2::new(500.0, 200.0))
                                    .with_size(Vec2::new(50.0, 50.0)),
                            ));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    };

    let mut app = EngineContext::new(
        &event_loop,
        |event, ch, assets, ph, command| window(event, ch, assets, ph, command),
        |ch, pc, assets, event, command| redraw_closure(ch, pc, assets, event, command),
    );
    event_loop.run_app(&mut app)
}
