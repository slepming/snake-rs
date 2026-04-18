use std::sync::Arc;

use crate::{ActiveEventLoop, EngineContext, drw::drawable::{Children, Drawable}, geometry, mv::transform::PhysicsContext};

use rapier2d::{math::Vec2, prelude::RigidBodyBuilder};
use winit::{event::WindowEvent, window::Window};

pub trait Game {
    fn start(&mut self, event_loop: &ActiveEventLoop, engine: &mut impl GameWrapper) -> Arc<Window>;
    fn update(&mut self, event_loop: &ActiveEventLoop, event: &WindowEvent, drws: &mut impl GameWrapper);
}

pub trait GameWrapper {
    fn get_children(&self) -> &Children;
    fn get_children_mut(&mut self) -> &mut Children;
    fn create_drawable(&mut self, shape: geometry::shapes::Shapes, start_position: Option<Vec2>);
    fn create_drawable_physics(&mut self, size: Vec2, start_position: Option<Vec2>, rigidbodybuilder: Option<RigidBodyBuilder>);
    fn physics_context_mut(&mut self) -> &mut PhysicsContext;
    fn physics_context(&self) -> &PhysicsContext;
}

impl<G> GameWrapper for EngineContext<G> where G: Game + Clone + Copy {
    fn get_children(&self) -> &Children {
        &self.children
    }

    fn get_children_mut(&mut self) -> &mut Children {
        &mut self.children
    }

    fn create_drawable(
        &mut self,
        shape: geometry::shapes::Shapes,
        start_position: Option<Vec2>,
    ) {
        self.children.add_drawable(Drawable::from_shape(
            shape,
            self.children.drawables.len() as u32 + self.children.physics_drawables.len() as u32 + 1,
            start_position,
        ));
    }

    fn create_drawable_physics(
        &mut self,
        size: Vec2,
        start_position: Option<Vec2>,
        rigidbodybuilder: Option<RigidBodyBuilder>,
    ) {
        self.children
            .add_physics(self.physics_context.create_phys_square(
                rigidbodybuilder.unwrap_or(RigidBodyBuilder::dynamic()),
                size.into(),
                self.children.physics_drawables.len() as u32
                    + self.children.drawables.len() as u32
                    + 1,
                start_position,
            ));
    }

    fn physics_context_mut(&mut self) -> &mut PhysicsContext {
        &mut self.physics_context
    }

    fn physics_context(&self) -> &PhysicsContext {
        &self.physics_context
    }
}
