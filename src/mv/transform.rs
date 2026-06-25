use rapier2d::{
    math::Vec2,
};

pub trait HasTransform {
    fn matrix_mut(&mut self) -> &mut [[f32; 4]; 4];
    fn matrix(&self) -> &[[f32; 4]; 4];
}

pub trait Positioned {
    fn position(&self) -> Vec2;
    fn set_position(&mut self, vec: Vec2);
}
