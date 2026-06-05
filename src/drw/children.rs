use std::slice::Iter;

use tracing::info;

use crate::drw::drawable::Drawable;

/// Structure which contains list of all objects
pub struct Children {
    /// List of all drawable objects
    drawables: Vec<Drawable>,
}

impl Children {
    /// Push drawable to the [`Children::drawables`]
    pub(crate) fn add(&mut self, item: Drawable) {
        self.drawables.push(item);
    }

    /// # Returns
    /// [`Drawable`]
    pub fn get(&self, index: usize) -> Option<&Drawable> {
        self.drawables.get(index)
    }

    /// # Returns
    /// [`Iter`]
    pub fn iter(&self) -> Iter<'_, Drawable> {
        self.drawables.iter()
    }

    /// # Returns
    /// Children length
    /// [`usize`]
    pub fn len(&self) -> usize {
        self.drawables.len()
    }

    pub(crate) fn clear(&mut self) {
        self.drawables.clear();
    }
}

impl Default for Children {
    fn default() -> Self {
        Self {
            drawables: Default::default(),
        }
    }
}
