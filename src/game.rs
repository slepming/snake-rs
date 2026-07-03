use std::sync::Arc;

use winit::dpi::PhysicalPosition;

use crate::{
    cmd::command::CommandQueue, drw::children::Children, fnt::font::TextFont, res::assets::Storage,
};

pub(crate) struct GameContext {
    pub children: Arc<Children>,
    pub assets: Storage,
    pub frames: u64,
    pub game_command_queue: CommandQueue,
    pub fonts: TextFont,
    pub mouse_position: Option<PhysicalPosition<f64>>,
}
