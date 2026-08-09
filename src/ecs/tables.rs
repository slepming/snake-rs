use crate::{
    Render,
    drw::drawable::DescriptorID,
    game::GameObject,
    geom::{matrix::Transform, shapes::Shapes},
    res::cache::{DescriptorSetCache, PipelineCache},
};
use hecs::{Bundle, ComponentError, Entity, World};
use std::{
    any::{TypeId, type_name},
    sync::Arc,
};
use vulkano::{
    descriptor_set::allocator::DescriptorSetAllocator, image::sampler::Sampler,
    memory::allocator::MemoryAllocator,
};

pub type DynObject = Box<dyn DynamicallyObjectAlias>;

pub trait DynamicallyObjectAlias: GameObject + Render + Send + Sync {}
impl<T> DynamicallyObjectAlias for T where T: GameObject + Render + Send + Sync {}

pub struct EntityComponent {
    pub(crate) world: World,
    memory_allocator: Arc<dyn MemoryAllocator>,
    descriptor_allocator: Arc<dyn DescriptorSetAllocator>,
    descriptor_cache: Arc<DescriptorSetCache>,
    pipeline_cache: Arc<PipelineCache>,
    sampler: Arc<Sampler>,
}

impl EntityComponent {
    pub(crate) fn new(
        memory_allocator: Arc<dyn MemoryAllocator>,
        descriptor_allocator: Arc<dyn DescriptorSetAllocator>,
        descriptor_cache: Arc<DescriptorSetCache>,
        pipeline_cache: Arc<PipelineCache>,
        sampler: Arc<Sampler>,
    ) -> Self {
        Self {
            world: World::new(),
            memory_allocator,
            descriptor_allocator,
            descriptor_cache,
            pipeline_cache,
            sampler,
        }
    }

    pub fn add<G>(&mut self, drw: G, transformation: Transform, shape: Shapes) -> Entity
    where
        G: GameObject + Render + Send + Sync + 'static,
    {
        let class = ClassInfo::of::<G>();

        shape.create_descriptor(
            DescriptorID::from(&class),
            self.memory_allocator.clone(),
            self.descriptor_allocator.clone(),
            self.descriptor_cache.clone(),
            self.pipeline_cache.clone(),
            Some(self.sampler.clone()),
        );

        let boxed_drw: DynObject = Box::new(drw);

        self.world.spawn((transformation, class, shape, boxed_drw)) // Transform,
        // DescriptorSet,
        // ClassInfo,
        // Shapes,
        // DynObject
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
