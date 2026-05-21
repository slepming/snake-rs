use std::sync::Arc;

use color::Rgba8;
use rand::RngExt;
use rapier2d::math::Vec2;
use snake_engine::{
    EngineContext,
    cmd::command::{CommandQueue, DrawCommand},
    drw::{
        drawable::{Children, DrawableCreateInfo},
        texture::Texture,
    },
    geom::shapes::Shapes,
    mv::phys::movement::PhysicsContext, res::assets::AssetsManager,
};
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
    let mut rng = rand::rng();

    let mut window =
        |e: &ActiveEventLoop, ch: &mut Children, wind: Arc<Window>, command: &mut CommandQueue| {
            let monitor_size = e.available_monitors().next().unwrap().size();
            wind.set_title("snake");
            for i in 0..OBJECTS_COUNT {
                let (r, g, b) = (rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>());
                let drawable_info = DrawableCreateInfo::default()
                    .with_size(Vec2::new(1000.0, 1000.0))
                    .with_color(Rgba8 { r, g, b, a: 255 })
                    .with_id(ch.drawables.len() as u32 + 1)
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
                        Shapes::Image(),
                        drawable_info.with_size(Vec2::new(50.0, 50.0)),
                    ));
                }
            }
        };

    let redraw_closure = |_ch: &mut Children,
                          _pc: &mut PhysicsContext,
                          event: &WindowEvent,
                          command: &mut CommandQueue| match event {
        WindowEvent::KeyboardInput { event, .. } => {
            let span = tracy_client::span!("Engine::Keyboard_input");
            span.emit_color(0xFF0000);
            if event.state == ElementState::Pressed && !event.repeat {
                match event.key_without_modifiers().as_ref() {
                    Key::Named(NamedKey::Escape) => {
                        //ch.physics_drawables.iter_mut().for_each(|r| {
                        //    if r.rigid_body(pc).is_dynamic() {
                        //        let object = pc.rigid_body_set[r.rb_handle()].clone();
                        //        r.teleport(
                        //            pc,
                        //            Vec2::new(
                        //                object.translation().x,
                        //                object.translation().y + 1000.0,
                        //            ),
                        //        );
                        //    }
                        //});
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };

    let mut app = EngineContext::new(
        &event_loop,
        |event, ch, ph, command| window(event, ch, ph, command),
        |ch, pc, event, command| redraw_closure(ch, pc, event, command),
    );
    event_loop.run_app(&mut app)
}
