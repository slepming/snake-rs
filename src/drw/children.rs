use std::slice::Iter;

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

    /// Check if drawable exists
    pub fn contains(&self, item: &Drawable) -> bool {
        self.drawables.contains(item)
    }

    /// Returns [`Drawable`]
    pub fn get(&self, index: usize) -> Option<&Drawable> {
        self.drawables.get(index)
    }

    /// Returns mutable [`Drawable`] reference
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Drawable> {
        self.drawables.get_mut(index)
    }

    /// Drawables iterator
    /// # Returns
    /// [`Iter`]
    pub fn iter(&self) -> Iter<'_, Drawable> {
        self.drawables.iter()
    }

    /// Returns the last element of the drawable, or `None` if it is empty.
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn last(&self) -> Option<&Drawable> {
        self.drawables.last()
    }

    /// Returns Children length
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
