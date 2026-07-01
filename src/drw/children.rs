use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};

use crate::drw::drawable::Drawable;

type DrawableData = Arc<Mutex<Drawable>>;

/// Structure which contains list of all objects
pub struct Children {
    /// List of all drawable objects
    drawables: RwLock<Vec<Arc<Mutex<Drawable>>>>,
}

impl Children {
    /// Locks for read drawables vector and executes fnmut with vector
    pub(crate) fn lock_read_and_execute<F>(&self, mut f: F)
        where F: FnMut(&RwLockReadGuard<'_, Vec<Arc<Mutex<Drawable>>>>)
    {
        let lock = self.drawables.read().unwrap();
        f(&lock);
        drop(lock);
    }
    
    /// Push drawable to the [`Children::drawables`]
    pub(crate) fn add(&self, item: Drawable) {
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
    pub fn get_mut(&self, index: usize) -> Option<DrawableData> {
        let lock = self.drawables.read().unwrap();

        lock.get(index).cloned()
    }

    /// Execute function for each drawable
    pub fn for_each<F>(&self, mut e: F)
    where
        F: FnMut((usize, &DrawableData)),
    {
        let lock = self.drawables.read().unwrap();
        for item in lock.iter().enumerate() {
            e(item);
        }
    }

    /// Filters drawables by predicate
    ///
    /// # Returns
    /// Vec<[`DrawableData`]>
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

    /// Clears everything drawable in vector
    pub(crate) fn clear(&self) {
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
