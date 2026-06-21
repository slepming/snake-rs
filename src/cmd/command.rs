//! Commands from game space

use std::{collections::VecDeque, sync::Arc};

use image::{ImageBuffer, Rgba};
use tracing::{debug, info, warn};
use vulkano::descriptor_set::DescriptorSet;

use crate::{
    EngineContext, RedrawFn, StartFn,
    drw::drawable::{Drawable, DrawableCreateInfo},
    geom::shapes::Shapes,
    res::cache::CacheProvider,
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
        #[cfg(feature = "tracing")]
        let span_submit = tracy_client::span!("Engine: Flush commands");

        if self.game.game_command_queue.commands.is_empty() {
            return;
        }

        let commands_count = self.game.game_command_queue.commands.len();
        debug!(commands_count = commands_count, "Flush commands");
        let commands: Vec<_> = self.game.game_command_queue.commands.drain(..).collect();
        for command in commands {
            match command {
                DrawCommand::DrawObject(s, drw) => {
                    let pipeline_name = s.as_ref().to_lowercase();
                    if let Some(drw) = draw_object(self, s, drw, false) {
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
                DrawCommand::ClearDrawables => {
                    info!("Clear drawables: {}", self.game.children.len());
                    self.game.children.clear();
                }
                DrawCommand::DrawText(text) => {
                    let drw_create_info = DrawableCreateInfo {
                        position: text.position,
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
                    if let Some(drw) = draw_object(self, s, drw_create_info, false) {
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

fn draw_object<Redraw, Start>(
    context: &EngineContext<Redraw, Start>,
    shape: Shapes,
    create_info: DrawableCreateInfo,
    force: bool,
) -> Option<(Drawable, Option<Arc<DescriptorSet>>)>
where
    Redraw: RedrawFn,
    Start: StartFn,
{
    let drw = Drawable::from_shape(
        shape.clone(),
        create_info.with_id(context.game.children.len() as u32 + 1),
        context.memory.memory_allocator.clone(),
        context.memory.descriptor_allocator.clone(),
        context.pipelines.clone(),
        context.descriptors.clone(),
        Some(context.sampler.clone()),
    );

    if context.game.children.contains(&drw.0) && !force {
        warn!("Object exists, dropping");
        drop(drw);
        return None;
    }

    Some(drw)
}
