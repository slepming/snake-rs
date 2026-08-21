use crate::{
    Render,
    drw::drawable::DescriptorID,
    game::GameObject,
    geom::{matrix::Transform, shapes::Shapes},
    res::cache::{DescriptorSetCache, PipelineCache},
};
use hecs::CommandBuffer;
pub use hecs::{Bundle, ComponentError, Entity, World};
use rayon::ThreadPool;
use std::{
    any::{TypeId, type_name},
    sync::Arc,
};
use tracing::debug;
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
    thread_pool: Arc<ThreadPool>,
    sampler: Arc<Sampler>,
}

impl EntityComponent {
    pub(crate) fn new(
        memory_allocator: Arc<dyn MemoryAllocator>,
        descriptor_allocator: Arc<dyn DescriptorSetAllocator>,
        descriptor_cache: Arc<DescriptorSetCache>,
        pipeline_cache: Arc<PipelineCache>,
        sampler: Arc<Sampler>,
        thread_pool: Arc<ThreadPool>
    ) -> Self {
        Self {
            buffer: CommandBuffer::new(),
            memory_allocator,
            descriptor_allocator,
            descriptor_cache,
            pipeline_cache,
            sampler,
            thread_pool
        }
    }

    pub fn add<G>(&mut self, drw: G, transformation: Transform)
    where
        G: GameObject + Render + Send + Sync + 'static,
    {
        let class = ClassInfo::of::<G>();

        let shape = drw.shape();

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

        let memory_allocator = self.memory_allocator.clone();
        let descriptor_set_allocator = self.descriptor_allocator.clone();
        let descriptor_set_cache = self.descriptor_cache.clone();
        let pipeline_cache = self.pipeline_cache.clone();
        let sampler = Some(self.sampler.clone());

        let descriptor_id = DescriptorID::from(&class);

        let shape_clone = shape.clone();
        // Likely someday it's will crash
        self.thread_pool.spawn(move ||{
            shape_clone.create_descriptor(
                descriptor_id,
                memory_allocator,
                descriptor_set_allocator,
                descriptor_set_cache,
                pipeline_cache,
                sampler
            );
        });

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
    pub(crate) fn attach_render_descriptor<G>(&mut self, entity: Entity, drw: G)
    where
        G: GameObject + Render + Send + Sync + 'static,
    {
        let class = ClassInfo::of::<G>();

        let shape = drw.shape();

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
    /// Returns [`ClassInfo`] with structure name as class name.
    pub fn of<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            class_name: type_name::<T>(),
        }
    }

    /// Returns [`ClassInfo`] with a given class name.
    ///
    /// It may create unique objects with unique descriptors.
    /// # Example
    ///
    /// ```rust
    /// #[game_object]
    /// struct SecondDrawable {}
    ///
    /// impl GameObject for SecondDrawable {
    ///     fn update(&mut self, _world: &mut EntityComponent) {}
    ///
    ///     fn start(&mut self, _world: &mut EntityComponent) {}
    /// }
    ///
    /// #[game_object]
    /// struct CustomDrawable {
    /// }
    ///
    /// impl GameObject for CustomDrawable {
    ///     fn update(&mut self, _world: &mut EntityComponent) {
    ///     }
    ///
    ///     fn start(&mut self, world: &mut EntityComponent) {
    ///         // Create an object with unique data
    ///         let drw = SecondDrawable {
    ///             color: Rgba8 {
    ///                 r: 255,
    ///                 g: 255,
    ///                 b: 255,
    ///                 a: 255,
    ///             },
    ///             shape: Shapes::Square(ShapeCreateInfo::default().with_radius(f32::MAX)),
    ///         };
    ///         let transform = Transform([
    ///             [50.0, 0.0, 0.0, 0.0],
    ///             [0.0, 50.0, 0.0, 0.0],
    ///             [0.0, 0.0, 1.0, 0.0],
    ///             [600.0, 600.0, 0.0, 1.0],
    ///         ]);
    ///
    ///         // Create a second object with unique data
    ///         let drw2 = SecondDrawable {
    ///             color: Rgba8 {
    ///                 r: 255,
    ///                 g: 0,
    ///                 b: 0,
    ///                 a: 255,
    ///             },
    ///             shape: Shapes::Square(ShapeCreateInfo::default().with_radius(0.3)),
    ///         };
    ///         let transform2 = Transform([
    ///             [50.0, 0.0, 0.0, 0.0],
    ///             [0.0, 50.0, 0.0, 0.0],
    ///             [0.0, 0.0, 1.0, 0.0],
    ///             [900.0, 900.0, 0.0, 1.0],
    ///         ]);
    ///
    ///         // Since the classes drw and drw2 are the same. Data between drw2 and drw are the
    ///         // same and last object, which will be added(with same classes), will create data for
    ///         // all objects of this class
    ///         // world.add(drw2, transform2);
    ///         // world.add(drw, transform);
    ///         // To avoid this we can use [`ClassInfo::of_class`] function
    ///
    ///         let shape = drw.shape();
    ///         let shape2 = drw2.shape();
    ///         let boxed_drw = Box::new(drw);
    ///         let boxed_drw2 = Box::new(drw2);
    ///         world.add_with_class(boxed_drw2, transform2, shape2, ClassInfo::of_class::<SecondDrawable>("drw2")); // <-- Here we create a unique class
    ///         world.add_with_class(boxed_drw, transform, shape, ClassInfo::of_class::<SecondDrawable>("drw")); // <-- Here we create a unique class
    ///     }
    /// }
    /// ```
    pub fn of_class<T: 'static>(class: &'static str) -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            class_name: class,
        }
    }
}
