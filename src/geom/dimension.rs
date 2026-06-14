use crate::Vector;

#[derive(Clone, Copy)]
pub struct Dimension {
    pub dimension: Vector
}

impl Dimension {
    pub fn new(d: Vector) -> Self {
        Self { dimension: d }
    }

    pub fn from_array(d: [f32; 2]) -> Self {
        Self { dimension: Vector::from_array(d) } 
    }
}

impl From<(u32, u32)> for Dimension {
    fn from(value: (u32, u32)) -> Self {
        Self { dimension: Vector::new(value.0 as f32, value.1 as f32) }
    }
}

impl From<(f32, f32)> for Dimension {
    fn from(value: (f32, f32)) -> Self {
        Self { dimension: Vector::new(value.0, value.1) }
    }
}
