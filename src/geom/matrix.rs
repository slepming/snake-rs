use std::hash::Hash;

use vulkano::buffer::BufferContents;

use crate::mv::transform::HasTransform;

#[repr(C)]
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct Transform(pub [[f32; 4]; 4]);

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.0[i][j].to_bits() != other.matrix()[i][j].to_bits() {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Transform {}

impl Hash for Transform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for row in self.matrix() {
            for &v in row {
                state.write_u32(v.to_bits());
            }
        }
    }
}

impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = format!(
            "\n{:?}\n{:?}\n{:?}\n{:?}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        );
        write!(f, "{}", fmt)
    }
}

impl HasTransform for Transform {
    fn matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.0
    }
    fn matrix(&self) -> &[[f32; 4]; 4] {
        &self.0
    }
}
