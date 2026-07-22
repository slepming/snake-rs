use std::sync::{Arc, Mutex};

use winit::dpi::PhysicalPosition;

use crate::{
    cmd::command::CommandQueue,
    drw::{
        children::Children,
        drawable::{Drawable, DrawableObjectFactory, ObjectFactory},
    },
    fnt::font::TextFont,
    res::assets::Storage,
};

pub(crate) struct GameContext {
    pub children: Arc<Children>,
    pub assets: Arc<Storage>,
    pub frames: u64,
    pub game_command_queue: CommandQueue,
    pub fonts: TextFont,
    pub mouse_position: Option<PhysicalPosition<f64>>,
}

// TODO является трейтом для создания возможности реализации кастомных игровых объектов
pub trait GameObject {
    fn drawables(&mut self) -> Option<Arc<Mutex<Drawable>>>;
    /// Executes before frame
    fn update(&mut self);

    fn start(&mut self, object_factory: DrawableObjectFactory);
}
