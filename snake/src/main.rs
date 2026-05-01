use std::sync::Arc;

use rapier2d::{math::Vec2, prelude::RigidBodyBuilder};
use snake_engine::{EngineContext, drw::drawable::{Children, Drawable}, mv::phys::movement::PhysicsContext};
use winit::{
    dpi::{PhysicalSize, Size},
    event::{ElementState, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
    platform::{
        modifier_supplement::KeyEventExtModifierSupplement, wayland::WindowAttributesExtWayland,
    },
    window::{Fullscreen, Window},
};
use color::Rgba8;

const OBJECTS_COUNT: u32 = 5;

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();

    let window = |e: &ActiveEventLoop, ch: &mut Children, pc: &mut PhysicsContext| {
        for i in 0..OBJECTS_COUNT {
            ch.add_drawable(Drawable::from_shape(snake_engine::geom::shapes::Shapes::Square([0.1, 0.1]),
                Rgba8 { r: 215, g: 230, b: 0, a: 255 },
                ch.physics_drawables.len() as u32 + ch.drawables.len() as u32 + 1,
                Some(Vec2::new(2.0 / i as f32, 0.0)),
            ));
        }
        Arc::new(
            e.create_window(
                Window::default_attributes()
                    .with_title("snake")
                    .with_name("snake-engine", "snake-engine")
                    .with_fullscreen(Some(Fullscreen::Borderless(None)))
                    .with_min_inner_size(Size::Physical(PhysicalSize {
                        width: 640,
                        height: 480,
                    }))
                    .with_max_inner_size(e.available_monitors().next().unwrap().size()),
            )
            .unwrap(),
        )
    };

    let redraw_closure =
        |ch: &mut Children, pc: &mut PhysicsContext, event: &WindowEvent| match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let span = tracy_client::span!("Engine::Keyboard_input");
                span.emit_color(0xFF0000);
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(NamedKey::Escape) => {
                            ch.physics_drawables.iter_mut().for_each(|r| {
                                if r.rigid_body(pc).is_dynamic() {
                                    let object = pc.rigid_body_set[r.rb_handle()].clone();
                                    r.teleport(
                                        pc,
                                        Vec2::new(
                                            object.translation().x,
                                            object.translation().y + 1000.0,
                                        ),
                                    );
                                }
                            });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };

    let mut app = EngineContext::new(
        &event_loop,
        |event, ch, ph| window(event, ch, ph),
        |ch, pc, event| redraw_closure(ch, pc, event),
    );
    event_loop.run_app(&mut app)
}
