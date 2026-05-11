use std::sync::Arc;

use color::Rgba8;
use rand::RngExt;
use rapier2d::math::Vec2;
use snake_engine::{
    EngineContext,
    drw::drawable::{Children, Drawable, DrawableCreateInfo},
    mv::phys::movement::PhysicsContext,
    res::cache::Cache,
};
use winit::{
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
};

const OBJECTS_COUNT: u32 = 6;

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    let mut rng = rand::rng();

    let mut window = |e: &ActiveEventLoop,
                      ch: &mut Children<Drawable>,
                      _pc: &mut PhysicsContext,
                      cache: Arc<Cache>| {
        let cache_clone = cache.clone();
        let monitor_size = e.available_monitors().next().unwrap().size();
        for i in 0..OBJECTS_COUNT {
            let (r, g, b) = (rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>());
            //dbg!((r, g, b));
            if i % 2 == 0 {
                ch.add_drawable(Drawable::from_shape(
                    snake_engine::geom::shapes::Shapes::Square(),
                    DrawableCreateInfo {
                        size: Vec2::new(100.0, 100.0),
                        color: Rgba8 { r, g, b, a: 255 },
                        id: ch.drawables.len() as u32 + 1,
                        cache: Some(cache_clone.clone()),
                        position: Some(Vec2::new(
                            300.0 * i as f32,
                            monitor_size.height as f32 / 2.0,
                        )),
                        ..Default::default()
                    },
                ));
            } else {
                ch.add_drawable(Drawable::from_shape(
                    snake_engine::geom::shapes::Shapes::Circle(),
                    DrawableCreateInfo {
                        size: Vec2::new(1000.0, 1000.0),
                        color: Rgba8 { r, g, b, a: 255 },
                        id: ch.drawables.len() as u32 + 1,
                        cache: Some(cache_clone.clone()),
                        position: Some(Vec2::new(
                            300.0 * i as f32,
                            monitor_size.height as f32 / 2.0,
                        )),
                        thickness: 0.0,
                        radius: 0.0,
                        ..Default::default()
                    },
                ));
            }
        }
    };

    let redraw_closure = |_ch: &mut Children<Drawable>,
                          _pc: &mut PhysicsContext,
                          event: &WindowEvent,
                          _cache: Arc<Cache>| match event {
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
        |event, ch, ph, cache| window(event, ch, ph, cache),
        |ch, pc, event, cache| redraw_closure(ch, pc, event, cache),
    );
    event_loop.run_app(&mut app)
}
