//! Commands from game space

use std::{collections::VecDeque, sync::Arc};

use image::{ImageBuffer, Rgba};
use tracing::{debug, info};
use vulkano::{descriptor_set::DescriptorSet, image::sampler::Sampler};

use crate::{
    EngineContext, RedrawFn, StartFn,
    drw::drawable::{Drawable, DrawableCreateInfo},
    geom::shapes::Shapes,
    mem::engine_memory::EngineMemory,
    res::cache::{CacheProvider, DescriptorSetCache, PipelineCache},
    text::sprite_text::SpriteTextCreateInfo,
};

pub enum DrawCommand {
    DrawObject(Shapes, DrawableCreateInfo),
    DrawText(SpriteTextCreateInfo),
    ClearDrawables,
}

pub enum DrawCommandReceive<'a> {
    Drawable(&'a Drawable),
}

/// FCFS Queue commands
pub struct CommandQueue {
    commands: VecDeque<DrawCommand>,
}

impl CommandQueue {
    /// Append command to the top of the queue
    pub fn append(&mut self, command: DrawCommand) {
        self.commands.push_back(command);
    }

    /// Returns command queue size
    /// # Returns
    /// [`usize`]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Returns true if commands queue size equals zero
    /// # Returns
    /// [`bool`]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Appends other commands to the top of queue
    pub fn append_other(&mut self, mut other: Self) {
        self.commands.append(&mut other.commands);
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self {
            commands: VecDeque::with_capacity(5),
        }
    }
}

pub trait CommandDispatcher {
    /// Executes each command from [`CommandQueue`]
    fn flush_commands(&mut self);
}

impl<Redraw, Start> CommandDispatcher for EngineContext<Redraw, Start>
where
    Redraw: RedrawFn,
    Start: StartFn,
{
    fn flush_commands(&mut self) {
        let _span_submit = tracy_client::span!("Engine: Flush commands");

        if self.game.game_command_queue.commands.is_empty() {
            return;
        }

        let commands_count = self.game.game_command_queue.commands.len();
        debug!(commands_count = commands_count, "Flush commands");
        let commands: Vec<_> = self.game.game_command_queue.commands.drain(..).collect();
        for command in commands {
            match command {
                DrawCommand::DrawObject(s, drw) => {
                    let memory = self.memory.clone();
                    let pipelines = self.pipelines.clone();
                    let descriptors = self.descriptors.clone();
                    let sampler = self.sampler.clone();
                    let c_len = self.game.children.count();
                    let children = self.game.children.clone();
                    self.thread_pool.spawn(move || {
                        let _span_submit = tracy_client::span!("Worker: Execute command");
                        let _pipeline_name = s.as_ref().to_lowercase();
                        if let Some(drw) = draw_object(
                            memory,
                            pipelines,
                            descriptors.clone(),
                            sampler,
                            s,
                            drw,
                            c_len,
                        ) {
                            if let Some(descriptor) = drw.1 {
                                if descriptors
                                    .get(drw.0.render.descriptor_id.id.clone().as_str())
                                    .is_none()
                                {
                                    descriptors.insert((
                                        drw.0.render.descriptor_id.id.clone(),
                                        descriptor.clone(),
                                    ));
                                }
                            }

                            children.add(drw.0);
                        }
                    });
                }
                DrawCommand::ClearDrawables => {
                    info!("Clear drawables: {}", self.game.children.count());
                    self.game.children.clear();
                }
                DrawCommand::DrawText(text) => {
                    let drw = DrawableCreateInfo {
                        position: text.position,
                        size: text.size,
                        ..Default::default()
                    };
                    let glyphs_image = self.game.fonts.get_glyphs(text);

                    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = glyphs_image.into_owned();

                    let mut png_bytes = Vec::new();
                    buffer
                        .write_to(
                            &mut std::io::Cursor::new(&mut png_bytes),
                            image::ImageFormat::Png,
                        )
                        .expect("Failed to write PNG");
                    let s = Shapes::Image(self.game.assets.load_texture_from_bytes(&png_bytes));
                    let pipeline_name = s.as_ref().to_lowercase();
                    let memory = self.memory.clone();
                    let pipelines = self.pipelines.clone();
                    let descriptors = self.descriptors.clone();
                    let sampler = self.sampler.clone();
                    let c_len = self.game.children.count();
                    if let Some(drw) = draw_object(
                        memory,
                        pipelines,
                        descriptors.clone(),
                        sampler,
                        s,
                        drw,
                        c_len,
                    ) {
                        if let Some(descriptor) = drw.1 {
                            if self
                                .descriptors
                                .get(pipeline_name.clone().as_str())
                                .is_none()
                            {
                                self.descriptors
                                    .insert((pipeline_name.clone(), descriptor.clone()));
                            }
                        }

                        let drw_id = drw.0.render.mesh.get_id().clone();
                        self.game.children.add(drw.0);
                        info!(
                            pipeline_name = &pipeline_name,
                            drw_id = drw_id,
                            "Object created"
                        );
                    }
                }
            }
        }
    }
}

fn draw_object(
    memory: Arc<EngineMemory>,
    pipelines: Arc<PipelineCache>,
    descriptors: Arc<DescriptorSetCache>,
    sampler: Arc<Sampler>,
    shape: Shapes,
    create_info: DrawableCreateInfo,
    children_len: usize,
) -> Option<(Drawable, Option<Arc<DescriptorSet>>)> {
    let drw = Drawable::from_shape(
        shape.clone(),
        create_info.with_id(children_len as u32 + 1),
        memory.memory_allocator.clone(),
        memory.descriptor_allocator.clone(),
        pipelines.clone(),
        descriptors.clone(),
        Some(sampler.clone()),
    );

    Some(drw)
}
