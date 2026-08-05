use hecs::{Bundle, ComponentError, Entity, World};

use crate::{DrawableRwLock, geom::matrix::Transform};

pub struct EntityComponent {
    pub(crate) world: World,
}

impl EntityComponent {
    pub(crate) fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn add(&mut self, drw: DrawableRwLock) -> Entity {
        self.world.spawn((Transform, drw))
    }

    pub fn remove<T>(&mut self, entity: Entity) -> Result<T, ComponentError>
    where
        T: Bundle + 'static,
    {
        self.world.remove(entity)
    }
}
