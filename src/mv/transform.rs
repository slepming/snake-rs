use crate::Vector;

pub trait HasTransform {
    fn matrix_mut(&mut self) -> &mut [[f32; 4]; 4];
    fn matrix(&self) -> &[[f32; 4]; 4];
}

pub trait Positioned {
    fn position(&self) -> Vector;
    fn set_position(&mut self, vec: Vector);
}
