use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::{EngineContext, drw::drawable::{Children, DrawableComponent, DrawableGPU}, mv::phys::movement::PhysicsContext};

enum DrawCommand {
    DrawObject()
}

pub struct CommandQueue {
    commands: Vec<DrawCommand>
}

impl CommandQueue {
    pub fn append(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self { commands: Vec::new() } 
    }
}

pub trait CommandDispatcher {
    fn flush_commands(&mut self, commands: Vec<DrawCommand>);
}

impl<Drw, Redraw, Start> CommandDispatcher for EngineContext<Drw, Redraw, Start>
where
    Drw: DrawableComponent + DrawableGPU + 'static,
    Redraw: FnMut(&mut Children<Drw>, &mut PhysicsContext, &WindowEvent, &mut CommandQueue),
    Start: FnMut(&ActiveEventLoop, &mut Children<Drw>, Arc<Window>, &mut CommandQueue),
{
    fn flush_commands(&mut self, commands: Vec<DrawCommand>) {
        for command in commands.iter() {
            match command
            {
                DrawCommand::DrawObject() => todo!(),
                _ => todo!()
            }
        }
    }
}
