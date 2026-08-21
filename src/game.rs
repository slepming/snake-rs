use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use hecs::World;
use rayon::ThreadPool;
use snake_macros::game_object;
use vulkano::{device::Queue, image::sampler::Sampler};
use winit::dpi::PhysicalPosition;

use crate::{
    Shapes,
    ecs::tables::{ClassInfo, DynObject, EntityComponent},
    fnt::font::TextFont,
    geom::{matrix::Transform, shapes::ShapeCreateInfo},
    mem::engine_memory::EngineMemory,
    res::{
        assets::Storage,
        cache::{DescriptorSetCache, PipelineCache},
    },
};

use color::Rgba8;

pub type Ecs = EntityComponent;

const BASIC_FONT: &'static str = "Fonts/freedom.otf";

pub(crate) struct GameContext {
    #[allow(dead_code)]
    pub assets: Arc<Storage>,
    pub frames: u64,
    #[allow(dead_code)]
    pub fonts: TextFont,
    pub mouse_position: Option<PhysicalPosition<f64>>,
    pub world: Arc<RwLock<World>>,
    pub world_buffer: EntityComponent,
}

impl GameContext {
    pub fn new(
        memory: Arc<EngineMemory>,
        queue: Arc<Queue>,
        pipelines: Arc<PipelineCache>,
        descriptors: Arc<DescriptorSetCache>,
        sampler: Arc<Sampler>,
        thread_pool: Arc<ThreadPool>
    ) -> Self {
        let world = Arc::new(RwLock::new(World::new()));

        let storage = Arc::new(Storage {
            queue: queue.clone(),
            memory_allocs: memory.clone(),
            texture_pool: RwLock::new(HashMap::new()),
        });

        let world_buffer = EntityComponent::new(
            memory.memory_allocator.clone(),
            memory.descriptor_allocator.clone(),
            descriptors.clone(),
            pipelines.clone(),
            sampler.clone(),
            thread_pool
        );

        Self {
            assets: storage,
            frames: 0,
            fonts: TextFont::new(BASIC_FONT),
            mouse_position: None,
            world: world,
            world_buffer: world_buffer,
        }
    }
}

/// Contains general functions for the operations of the object
pub trait GameObject {
    /// Executes every frame
    fn update(&mut self, world: &mut EntityComponent);

    fn start(&mut self, world: &mut EntityComponent);
}

pub enum CanvasCommand {
    CreateObject {
        object: DynObject,
        transform: Transform,
        shape: Shapes,
        class: ClassInfo,
    },
}

/// The main structure, handle first object structure from user.
#[game_object]
pub struct Canvas {
    pub buffer: Vec<CanvasCommand>,
}

impl Canvas {
    pub fn new(rgba: Rgba8) -> Self {
        Self {
            buffer: Vec::new(),
            color: rgba,
            shape: Shapes::Square(ShapeCreateInfo::default()),
        }
    }
}

impl GameObject for Canvas {
    fn update(&mut self, _world: &mut EntityComponent) {}

    fn start(&mut self, world: &mut EntityComponent) {
        for command in self.buffer.drain(..) {
            match command {
                CanvasCommand::CreateObject {
                    object,
                    transform,
                    shape,
                    class,
                } => world.add_with_class(object, transform, shape, class),
            }
        }
    }
}
