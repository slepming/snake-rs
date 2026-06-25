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
        let vector_position = position.into_vector();
        self.iter().filter(|&d| into_range(d, vector_position))
            .collect()
    }
}

/// Calculate drawable is into position range
fn into_range(drawable: &Drawable, position: Vector) -> bool {
    let drawable_size = drawable.size();
    let drawable_position = drawable.position();
    let drawable_sum = drawable_position + drawable_size;
    let drawable_position_gt = position.element_sum() >= drawable_position.element_sum();
    let drawable_position_lt = position.element_sum() <= drawable_sum.element_sum();
    debug!("{:?} > {:?} = {:?}; {:?} < {:?} = {:?}", position, drawable_position, drawable_position_gt, position, drawable_sum, drawable_position_lt);
    drawable_position_gt && drawable_position_lt
}

impl IntoVector for PhysicalPosition<f64> {
    fn into_vector(self) -> Vector {
        Vector {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
