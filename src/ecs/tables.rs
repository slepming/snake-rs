use crate::{
    Render,
    drw::drawable::DescriptorID,
    game::GameObject,
    geom::{matrix::Transform, shapes::Shapes},
    res::cache::{DescriptorSetCache, PipelineCache},
};
use hecs::CommandBuffer;
pub use hecs::{Bundle, ComponentError, Entity, World};
use tracing::debug;
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
    pub(crate) buffer: CommandBuffer,
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
            buffer: CommandBuffer::new(),
            memory_allocator,
            descriptor_allocator,
            descriptor_cache,
            pipeline_cache,
            sampler,
        }
    }

    pub fn add<G>(&mut self, drw: G, transformation: Transform, shape: Shapes)
    where
        G: GameObject + Render + Send + Sync + 'static,
    {
        let class = ClassInfo::of::<G>();

        let boxed_drw: DynObject = Box::new(drw);

        self.push(boxed_drw, transformation, shape, class);
        // DescriptorSet,
        // ClassInfo,
        // Shapes,
        // DynObject
    }

    pub fn add_with_class(
        &mut self,
        drw: DynObject,
        transformation: Transform,
        shape: Shapes,
        class: ClassInfo,
    ) {
        self.push(drw, transformation, shape, class);
    }

    fn push(&mut self, drw: DynObject, transformation: Transform, shape: Shapes, class: ClassInfo) {
        debug!("{:?}", class);
        shape.create_descriptor(
            DescriptorID::from(&class),
            self.memory_allocator.clone(),
            self.descriptor_allocator.clone(),
            self.descriptor_cache.clone(),
            self.pipeline_cache.clone(),
            Some(self.sampler.clone()),
        );

        self.buffer.spawn((transformation, class, shape, drw));
    }

    pub fn remove<T>(&mut self, entity: Entity)
    where
        T: Bundle + 'static,
    {
        self.buffer.remove::<T>(entity);
    }

    #[allow(dead_code)]
    /// Update [`Entity`] through buffer
    pub(crate) fn attach_render_descriptor<G>(&mut self, entity: Entity, drw: G, shape: Shapes)
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

        self.buffer.insert(entity, (shape, boxed_drw));
    }

    /// Executes commands from buffer immediately.
    ///
    /// # Note
    /// May cause unexpected errors
    pub fn execute_commands(&mut self, world: &mut World) {
        self.buffer.run_on(world);
    }
}

#[derive(Debug)]
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

    pub fn of_class<T: 'static>(class: &'static str) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            class_name: class,
        }
    }
}
