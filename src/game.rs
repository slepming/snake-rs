use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use hecs::{Entity, World};
use vulkano::{device::Queue, image::sampler::Sampler};
use winit::dpi::PhysicalPosition;

use crate::{
    ecs::tables::EntityComponent,
    fnt::font::TextFont,
    mem::engine_memory::EngineMemory,
    res::{
        assets::Storage,
        cache::{DescriptorSetCache, PipelineCache},
    },
};

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
    /// Main entity that have size equals window size
    pub entity: Option<Entity>,
}

impl GameContext {
    pub fn new(
        memory: Arc<EngineMemory>,
        queue: Arc<Queue>,
        pipelines: Arc<PipelineCache>,
        descriptors: Arc<DescriptorSetCache>,
        sampler: Arc<Sampler>,
    ) -> Self {
        let world = Arc::new(RwLock::new(World::new()));

        let storage = Arc::new(Storage {
            queue: queue.clone(),
            memory_allocs: memory.clone(),
            texture_pool: RwLock::new(HashMap::new()),
        });

        let world_buffer = EntityComponent::new(
            world.clone(),
            memory.memory_allocator.clone(),
            memory.descriptor_allocator.clone(),
            descriptors.clone(),
            pipelines.clone(),
            sampler.clone(),
        );

        Self {
            assets: storage,
            frames: 0,
            fonts: TextFont::new(BASIC_FONT),
            mouse_position: None,
            world: world,
            world_buffer: world_buffer,
            entity: None,
        }
    }
}

/// Contains general functions for the operations of the object
pub trait GameObject {
    /// Executes every frame
    fn update(&mut self, world: &mut EntityComponent);

    fn start(&mut self, world: &mut EntityComponent);
}
