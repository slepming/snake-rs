use std::sync::Arc;

use tracing::info;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::{
    EngineContext,
    drw::drawable::{Children, Drawable, DrawableCreateInfo},
    geom::shapes::Shapes,
    mv::phys::movement::PhysicsContext,
    res::cache::CacheProvider,
};

pub enum DrawCommand {
    DrawObject(Shapes, DrawableCreateInfo),
}

pub enum DrawCommandReceive<'a> {
    Drawable(&'a Drawable),
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
    fn flush_commands(&mut self, commands: CommandQueue);
}

impl<Redraw, Start> CommandDispatcher for EngineContext<Redraw, Start>
where
    Redraw: FnMut(&mut Children, &mut PhysicsContext, &WindowEvent, &mut CommandQueue),
    Start: FnMut(&ActiveEventLoop, &mut Children, Arc<Window>, &mut CommandQueue),
{
    fn flush_commands(&mut self, queue: CommandQueue) {
        #[cfg(feature = "tracing")]
        let span_submit = tracy_client::span!("Engine: Flush commands");
        info!("Flush commands");
        let commands = queue.commands;
        for command in commands.into_iter() {
            match command {
                DrawCommand::DrawObject(s, drw) => {
                    let pipeline_name = s.as_ref().to_lowercase();
                    let drw = Drawable::from_shape(
                        s.clone(),
                        drw,
                        self.pipelines
                            .get(&pipeline_name)
                            .expect("There is no pipeline for this drawable"),
                        self.memory.memory_allocator.clone(),
                        self.memory.descriptor_allocator.clone(),
                        Some(self.sampler.clone()),
                    );
                    self.descriptors
                        .insert((pipeline_name, drw.1.unwrap().clone()));
                    self.children.add_drawable(drw.0);
                }
            }
        }
    }
}
