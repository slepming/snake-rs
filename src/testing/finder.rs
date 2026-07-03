//! Find and get objects

use std::sync::{Arc, Mutex};

use tracing::debug;
use winit::dpi::PhysicalPosition;

use crate::{
    Vector,
    drw::{
        children::Children,
        drawable::{Drawable, DrawableComponent},
    },
    mv::transform::Positioned,
};

pub trait IntoVector {
    fn into_vector(self) -> Vector;
}

pub trait Finder {
    fn get_by_position(&self, position: PhysicalPosition<f64>) -> Vec<Arc<Mutex<Drawable>>>;
}

impl Finder for Children {
    /// Calculates all drawable position and compares with the specified range
    ///
    /// # Returns
    /// [`Drawable`] Vector
    ///
    /// # Arguments
    /// `position` - Position at which the object is located.
    fn get_by_position(&self, position: PhysicalPosition<f64>) -> Vec<Arc<Mutex<Drawable>>> {
        let vector_position = position.into_vector();
        self.filter_each(|d| into_range(d, vector_position))
    }
}

/// Calculate drawable is into position range
fn into_range(drawable: &Drawable, position: Vector) -> bool {
    let drawable_size = drawable.size();
    let drawable_position = drawable.position();
    let drawable_max = drawable_position + drawable_size;

    let inside_x = position.x >= drawable_position.x && position.x <= drawable_max.x;
    let inside_y = position.y >= drawable_position.y && position.y <= drawable_max.y;

    let ge_check = position.cmpge(drawable_position);
    let le_check = position.cmple(drawable_max);
    let inside = ge_check.all() && le_check.all();

    debug!("--- checking object ---");
    debug!("cursor: {:?}", position);
    debug!(
        "object position (min): {:?}, object size: {:?}",
        drawable_position, drawable_size
    );
    debug!("object max bounds: {:?}", drawable_max);
    debug!(
        "axis x check ({:.1} <= {:.1} <= {:.1}): {}",
        drawable_position.x, position.x, drawable_max.x, inside_x
    );
    debug!(
        "axis y check ({:.1} <= {:.1} <= {:.1}): {}",
        drawable_position.y, position.y, drawable_max.y, inside_y
    );
    debug!("final result -> inside: {}", inside);

    inside
}

impl IntoVector for PhysicalPosition<f64> {
    fn into_vector(self) -> Vector {
        Vector {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
