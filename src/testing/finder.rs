//! Find and get objects

use tracing::debug;
use winit::dpi::PhysicalPosition;

use crate::{
    Vector,
    drw::{children::Children, drawable::{Drawable, DrawableComponent}},
    mv::transform::Positioned,
};

pub trait IntoVector {
    fn into_vector(self) -> Vector;
}

pub trait Finder {
    fn get_by_position(&self, position: PhysicalPosition<f64>) -> Vec<&Drawable>;
}

impl Finder for Children {
    /// Calculates all drawable position and compares with the specified range
    ///
    /// # Returns
    /// [`Drawable`] Vector
    ///
    /// # Arguments
    /// `position` - Position at which the object is located.
    fn get_by_position(&self, position: PhysicalPosition<f64>) -> Vec<&Drawable> {
        self.iter()
            .for_each(|d| { debug!("drawable position: {}, drawable size: {}", d.position(), d.transform()); });
        self.iter().filter(|d| d.position() == position.into_vector())
            .collect()
    }
}

impl IntoVector for PhysicalPosition<f64> {
    fn into_vector(self) -> Vector {
        Vector {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
