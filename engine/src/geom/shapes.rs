use std::sync::Arc;
use strum::{AsRefStr, IntoStaticStr};
use tracing::warn;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::descriptor_set::allocator::DescriptorSetAllocator;
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::image::sampler::Sampler;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter};
use vulkano::pipeline::Pipeline;

use crate::MyVertex;
use crate::drw::texture::Texture;
use crate::res::assets::TextureHandler;

#[derive(vulkano::buffer::BufferContents, Clone, Copy)]
#[repr(C)]
pub struct CircleData {
    pub radius: f32,
    pub thickness: f32,
}

#[derive(AsRefStr, IntoStaticStr, Clone)]
pub enum Shapes {
    Square,
    Circle,
    Image(TextureHandler),
}

impl Shapes {
    /// Creates vertices and descriptor set
    /// # Returns
    /// Vertex and optional descriptor set
    pub fn get_vertex_and_descriptor(
        &self,
        pipeline: Arc<dyn Pipeline>,
        memory_allocator: Arc<dyn MemoryAllocator>,
        descriptor_allocator: Arc<dyn DescriptorSetAllocator>,
        sampler: Option<Arc<Sampler>>,
    ) -> (Vec<MyVertex>, Option<Arc<DescriptorSet>>) {
        match self {
            Shapes::Square => {
                let verts = vec![
                    MyVertex {
                        position: [-1.0, -1.0],
                    },
                    MyVertex {
                        position: [1.0, -1.0],
                    },
                    MyVertex {
                        position: [1.0, 1.0],
                    },
                    MyVertex {
                        position: [-1.0, 1.0],
                    },
                ];
                (verts, None)
            }
            Shapes::Circle => {
                let verts = vec![
                    MyVertex {
                        position: [-1.0, -1.0],
                    },
                    MyVertex {
                        position: [1.0, -1.0],
                    },
                    MyVertex {
                        position: [1.0, 1.0],
                    },
                    MyVertex {
                        position: [-1.0, 1.0],
                    },
                ];

                let buffer = Buffer::from_data(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::UNIFORM_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    CircleData {
                        radius: 0.05,
                        thickness: 0.001,
                    },
                )
                .unwrap();

                let layout = pipeline.layout().set_layouts().get(0).unwrap().clone();
                if layout.bindings().is_empty() {
                    warn!("Pipeline 'circle' has no bindings. Did you forget to compile shaders?");
                }

                let descriptor_set = DescriptorSet::new(
                    descriptor_allocator.clone(),
                    layout,
                    [WriteDescriptorSet::buffer(0, buffer)],
                    [],
                )
                .unwrap();

                (verts, Some(descriptor_set))
            }
            Shapes::Image(texture) => {
                let verts = vec![
                    MyVertex {
                        position: [-1.0, -1.0],
                    },
                    MyVertex {
                        position: [1.0, -1.0],
                    },
                    MyVertex {
                        position: [1.0, 1.0],
                    },
                    MyVertex {
                        position: [-1.0, 1.0],
                    },
                ];

                let layout = pipeline.layout().set_layouts().get(0).unwrap().clone();
                if layout.bindings().is_empty() {
                    warn!("Pipeline 'image' has no bindings. Did you forget to compile shaders?");
                }
                let image_view = texture.view.clone();

                let descriptor_set = DescriptorSet::new(
                    descriptor_allocator.clone(),
                    layout,
                    [WriteDescriptorSet::image_view_sampler(
                        0,
                        image_view,
                        sampler.unwrap().clone(),
                    )],
                    [],
                )
                .unwrap();

                (verts, Some(descriptor_set))
            }
        }
    }
}
