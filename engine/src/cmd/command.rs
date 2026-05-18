use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::{
    EngineContext, MyVertex,
    drw::drawable::{Children, Drawable, DrawableComponent, DrawableCreateInfo, DrawableGPU},
    geom::shapes::Shapes,
    mv::phys::movement::PhysicsContext,
    res::cache::CacheProvider,
};

enum DrawCommand {
    DrawObject(Shapes, DrawableCreateInfo),
}

enum DrawCommandReceive {
    Drawable(Drawable),
}

pub struct CommandQueue {
    commands: Vec<DrawCommand>,
}

impl CommandQueue {
    pub fn append(&mut self, command: DrawCommand) {
        self.commands.push(command);
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

pub trait CommandDispatcher {
    fn flush_commands(&mut self, commands: Vec<DrawCommand>) -> Option<DrawCommandReceive>;
}

impl<Drw, Redraw, Start> CommandDispatcher for EngineContext<Drw, Redraw, Start>
where
    Drw: DrawableComponent + DrawableGPU + 'static,
    Redraw: FnMut(&mut Children<Drw>, &mut PhysicsContext, &WindowEvent, &mut CommandQueue),
    Start: FnMut(&ActiveEventLoop, &mut Children<Drw>, Arc<Window>, &mut CommandQueue),
{
    fn flush_commands(&mut self, commands: Vec<DrawCommand>) -> Option<DrawCommandReceive> {
        for command in commands.into_iter() {
            match command {
                DrawCommand::DrawObject(s, drw) => {
                    let drw = Drawable::from_shape(
                        s.clone(),
                        drw,
                        self.pipelines.get(s.clone().into()).unwrap().clone(),
                        self.memory.memory_allocator.clone(),
                        self.memory.descriptor_allocator.clone(),
                        Some(self.sampler.clone()),
                    );
                    let key: &str = s.into();
                    self.descriptors
                        .insert((key.to_string(), drw.1.unwrap().clone()));
                    return Some(DrawCommandReceive::Drawable(drw.0));
                }
                _ => return None,
            }
        }
        None
    }
}
