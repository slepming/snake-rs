//! Find and get objects

use winit::dpi::PhysicalPosition;

use crate::Vector;

pub trait IntoVector {
    fn into_vector(self) -> Vector;
}

pub trait Finder {
    fn get_by_position(&self, position: PhysicalPosition<f64>);
}

#[allow(dead_code)]
/// Calculate drawable is into position range
fn into_range() -> bool {
    //let drawable = drw.read().unwrap();
    //let size = drawable.size();
    //let pos = drawable.position();

    //let half_size = size / 2.0;
    //let min_bound = pos - half_size;
    //let max_bound = pos + half_size;

    //let inside_x = position.x >= min_bound.x && position.x <= max_bound.x;
    //let inside_y = position.y >= min_bound.y && position.y <= max_bound.y;
    //let inside = inside_x && inside_y;

    //debug!(
    //    "AABB Check | Cursor: [x: {:.1}, y: {:.1}] | Bounds: X[{:.1}..{:.1}] Y[{:.1}..{:.1}] | Result: {}",
    //    position.x, position.y, min_bound.x, max_bound.x, min_bound.y, max_bound.y, inside
    //);

    //drop(drawable);

    //inside
    false
}

impl IntoVector for PhysicalPosition<f64> {
    fn into_vector(self) -> Vector {
        Vector {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}
