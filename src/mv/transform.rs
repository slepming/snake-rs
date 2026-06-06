use rapier2d::{
    math::Vec2,
    prelude::{RigidBody, RigidBodyHandle},
};

use crate::{
    drw::drawable::DrawableComponent,
    mv::phys::movement::{DynamicObject, PhysicsContext},
};

pub trait HasTransform {
    fn matrix_mut(&mut self) -> &mut [[f32; 4]; 4];
    fn matrix(&self) -> &[[f32; 4]; 4];
}

pub trait Entity: DrawableComponent + DynamicObject {
    fn rigid_body<'a>(&self, ctx: &'a mut PhysicsContext) -> &'a mut RigidBody;

    fn rb_handle(&self) -> RigidBodyHandle;
}

pub trait Positioned {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, vec: Vec2);
}
