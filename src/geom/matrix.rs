use std::hash::Hash;

use vulkano::buffer::BufferContents;

use crate::mv::transform::HasTransform;

#[repr(C)]
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct Transform {
    pub transform: [[f32; 4]; 4],
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.transform[i][j].to_bits() != other.transform[i][j].to_bits() {
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
        for row in &self.transform {
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
            self.transform[0], self.transform[1], self.transform[2], self.transform[3]
        );
        write!(f, "{}", fmt)
    }
}

impl HasTransform for Transform {
    fn matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
    }
    fn matrix(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }
}
