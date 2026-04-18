use std::{error::Error, sync::Arc};

use rapier2d::{math::Vec2, prelude::RigidBodyBuilder};
use snake_engine::{EngineContext, drw::drawable::Children, game::GameWrapper};
use winit::{
    dpi::{PhysicalSize, Size},
    event::ElementState,
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    platform::{
        modifier_supplement::KeyEventExtModifierSupplement, wayland::WindowAttributesExtWayland,
    },
    window::Window,
};

const OBJECTS_COUNT: u32 = 5;

fn main() -> Result<(), impl Error> {
    let snake = Snake {};
    let event_loop = EventLoop::new().unwrap();
    let mut app = EngineContext::new(&event_loop, snake);
    event_loop.run_app(&mut app)
}

#[derive(Copy, Clone)]
struct Snake {}

impl snake_engine::game::Game for Snake {
    fn start(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        drw: &mut impl GameWrapper,
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
        drws: &mut impl GameWrapper,
    ) {
        // TODO: Надо чтобы был какой-то трейт между update и Drawables и EngineContext.(трейт для
        // EngineContext, в котором есть Drawables
        let children: &mut Children = drws.as_ref().get_children_mut();
        match event.clone() {
            winit::event::WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let span = tracy_client::span!("Engine::Keyboard_input");
                span.emit_color(0xFF0000);
                if event.state == ElementState::Pressed && !event.repeat {
                    match event.key_without_modifiers().as_ref() {
                        Key::Named(NamedKey::Escape) => {
                            children.physics_drawables.iter_mut().for_each(|r| {
                                let cont = drws.physics_context_mut();
                                if r.rigid_body(cont).is_dynamic() {
                                    let object = cont.rigid_body_set[r.rb_handle()].clone();
                                    r.teleport(
                                        cont,
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
        }
        todo!()
    }
}
