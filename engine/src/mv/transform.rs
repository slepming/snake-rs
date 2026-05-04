use rapier2d::prelude::{RigidBody, RigidBodyHandle};

use crate::{
    drw::drawable::DrawableGPU,
    mv::phys::movement::{DynamicObject, PhysicsContext},
};

pub trait Position {
    fn get_matrix_mut(&mut self) -> &mut [[f32; 4]; 4];
    fn get_matrix(&self) -> &[[f32; 4]; 4];
}

pub trait Entity: DrawableGPU + DynamicObject {
    fn rigid_body<'a>(&self, ctx: &'a mut PhysicsContext) -> &'a mut RigidBody;

    fn rb_handle(&self) -> RigidBodyHandle;
}
