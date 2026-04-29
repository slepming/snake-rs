use vulkano::buffer::BufferContents;

use crate::mv::transform::Position;

#[repr(C)]
#[derive(BufferContents, Clone, Copy, Debug)]
pub struct Transform {
    pub transform: [[f32; 4]; 4],
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

impl Position for Transform {
    fn get_matrix_mut(&mut self) -> &mut [[f32; 4]; 4] {
        &mut self.transform
    }
    fn get_matrix(&self) -> &[[f32; 4]; 4] {
        &self.transform
    }
}
