use std::sync::Arc;

use tracing::info;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::Window};

use crate::{
    EngineContext, cmd::command, drw::drawable::{Children, Drawable, DrawableCreateInfo}, geom::shapes::Shapes, mv::phys::movement::PhysicsContext, res::cache::CacheProvider
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

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
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
    Redraw: FnMut(
        &mut Children,
        &mut PhysicsContext,
        &mut crate::res::assets::AssetsManager,
        &WindowEvent,
        &mut CommandQueue,
    ),
    Start: FnMut(
        &ActiveEventLoop,
        &mut Children,
        &mut crate::res::assets::AssetsManager,
        Arc<Window>,
        &mut CommandQueue,
    ),
{
    fn flush_commands(&mut self, queue: CommandQueue) {
        #[cfg(feature = "tracing")]
        let span_submit = tracy_client::span!("Engine: Flush commands");
        if queue.commands.is_empty() {
            return;
        }

        let commands_count = queue.commands.len();
        info!(commands_count = commands_count, "Flush commands");
        let commands = queue.commands.into_iter();
        for command in commands {
            match command {
                DrawCommand::DrawObject(s, drw) => {
                    let pipeline_name = s.as_ref().to_lowercase();
                    let drw = Drawable::from_shape(
                        s.clone(),
                        drw,
                        self.memory.memory_allocator.clone(),
                        self.memory.descriptor_allocator.clone(),
                        self.pipelines.clone(),
                        self.descriptors.clone(),
                        Some(self.sampler.clone()),
                    );

                    if let Some(descriptor) = drw.1 {
                        self.descriptors.insert((pipeline_name, descriptor.clone()));
                    }

                    self.children.add_drawable(drw.0);
                }
            }
        }
    }
}
