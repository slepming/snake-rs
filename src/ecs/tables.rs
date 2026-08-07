use crate::{Render, game::GameObject, geom::matrix::Transform};
use hecs::{Bundle, ComponentError, Entity, World};
use std::any::{TypeId, type_name};

pub type DynObject = Box<dyn DynamicallyObjectAlias>;

pub trait DynamicallyObjectAlias: GameObject + Render + Send + Sync {}
impl<T> DynamicallyObjectAlias for T where T: GameObject + Render + Send + Sync {}

pub struct EntityComponent {
    pub(crate) world: World,
}

impl EntityComponent {
    pub(crate) fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn add<G>(&mut self, drw: G) -> Entity
    where
        G: GameObject + Render + Send + Sync + Copy + 'static,
    {
        let class = ClassInfo::of::<G>();

        let boxed_drw: DynObject = Box::new(drw);

        self.world.spawn((Transform, class, boxed_drw))
    }

    pub fn remove<T>(&mut self, entity: Entity) -> Result<T, ComponentError>
    where
        T: Bundle + 'static,
    {
        self.world.remove(entity)
    }
}

pub struct ClassInfo {
    pub type_id: TypeId,
    pub class_name: &'static str,
}

impl ClassInfo {
    pub fn of<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            class_name: type_name::<T>(),
        }
    }
}
