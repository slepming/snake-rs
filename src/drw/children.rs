use std::sync::{Arc, Mutex, RwLock};

use crate::drw::drawable::Drawable;

type DrawableData = Arc<Mutex<Drawable>>;

/// Structure which contains list of all objects
pub struct Children {
    /// List of all drawable objects
    drawables: RwLock<Vec<Arc<Mutex<Drawable>>>>,
}

impl Children {
    /// Push drawable to the [`Children::drawables`]
    pub(crate) fn add(&mut self, item: Drawable) {
        self.drawables
            .write()
            .unwrap()
            .push(Arc::new(Mutex::new(item)));
    }

    /// Check if arc exists
    pub fn contains_arc(&self, item: &DrawableData) -> bool {
        self.drawables
            .read()
            .unwrap()
            .iter()
            .any(|d| Arc::ptr_eq(d, item))
    }

    /// Returns [`Drawable`]
    pub fn get(&self, index: usize) -> Option<DrawableData> {
        self.drawables.read().unwrap().get(index).cloned()
    }

    /// Returns mutable [`Drawable`] reference
    pub fn get_mut(&mut self, index: usize) -> Option<DrawableData> {
        let lock = self.drawables.read().unwrap();

        lock.get(index).cloned()
    }

    pub fn for_each<F>(&self, mut e: F)
    where
        F: FnMut((usize, &DrawableData)),
    {
        let lock = self.drawables.read().unwrap();
        for item in lock.iter().enumerate() {
            e(item);
        }
    }

    pub fn filter_each<P>(&self, mut predicate: P) -> Vec<DrawableData>
    where
        P: FnMut(&Drawable) -> bool,
    {
        let lock = self.drawables.read().unwrap();

        lock.iter()
            .filter(|item| {
                let guard = item.lock().unwrap();
                predicate(&guard)
            })
            .cloned()
            .collect()
    }

    /// Returns the last element of the drawable, or `None` if it is empty.
    ///
    /// # Returns
    /// [`Drawable`]
    pub fn last(&self) -> Option<DrawableData> {
        self.drawables.read().unwrap().last().cloned()
    }

    /// Returns Children length
    pub fn len(&self) -> usize {
        self.drawables.read().unwrap().len()
    }

    pub(crate) fn clear(&mut self) {
        self.drawables.write().unwrap().clear();
    }
}

impl Default for Children {
    fn default() -> Self {
        Self {
            drawables: Default::default(),
        }
    }
}
