use std::sync::{Arc, RwLock};

use winit::dpi::PhysicalPosition;

use crate::{
    DrawableRwLock, cmd::command::CommandQueue, drw::{children::Children, drawable::DrawableObjectFactory}, ecs::tables::EntityComponent, fnt::font::TextFont, res::assets::Storage
};

pub type Ecs = Arc<RwLock<EntityComponent>>;

pub(crate) struct GameContext {
    pub children: Arc<Children>,
    pub assets: Arc<Storage>,
    pub frames: u64,
    pub game_command_queue: CommandQueue,
    #[allow(dead_code)]
    pub fonts: TextFont,
    pub mouse_position: Option<PhysicalPosition<f64>>,
    pub drawable_object_factory: Arc<DrawableObjectFactory>,
    pub world: Ecs
}

// TODO является трейтом для создания возможности реализации кастомных игровых объектов
pub trait GameObject {
    fn drawables(&self) -> DrawableRwLock;
    /// Executes before frame
    fn update(&mut self, world: Ecs);

    fn start(&mut self, object_factory: Arc<DrawableObjectFactory>);
}
