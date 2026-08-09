use std::sync::{Arc, RwLock};

use winit::dpi::PhysicalPosition;

use crate::{
    drw::children::Children, ecs::tables::EntityComponent, fnt::font::TextFont,
    res::assets::Storage,
};

pub type Ecs = Arc<RwLock<EntityComponent>>;

pub(crate) struct GameContext {
    pub children: Arc<Children>,
    #[allow(dead_code)]
    pub assets: Arc<Storage>,
    pub frames: u64,
    #[allow(dead_code)]
    pub fonts: TextFont,
    pub mouse_position: Option<PhysicalPosition<f64>>,
    pub world: Ecs,
}

/// Contains general functions for the operations of the object
pub trait GameObject {
    /// Executes every frame
    fn update(&mut self, world: Ecs);

    fn start(&mut self, world: Ecs);
}
