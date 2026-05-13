use std::sync::Arc;
use strum::IntoStaticStr;
use tracing::warn;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::image::view::{ImageView, ImageViewCreateInfo};
use vulkano::image::{Image, ImageCreateInfo, ImageSubresourceRange, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::pipeline::Pipeline;

use crate::MyVertex;
use crate::drw::texture::Texture;
use crate::res::cache::{Cache, PipelineHandle};

#[derive(vulkano::buffer::BufferContents, Clone, Copy)]
#[repr(C)]
pub struct CircleData {
    pub radius: f32,
    pub thickness: f32,
}

#[derive(IntoStaticStr, Clone)]
pub enum Shapes {
    Square,
    Circle,
    Image(Texture),
}

impl Shapes {
    /// Creates vertices and descriptor set
    /// # Returns
    /// Vertex and optional descriptor set
    pub fn get_vertex_and_descriptor(
        &self,
        cache: &Arc<Cache>,
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

                let memory_allocator = cache.memory_allocator.as_ref().unwrap();
                let descriptor_allocator = cache.descriptor_allocator.as_ref().unwrap();
                let pipeline = cache.get_pipeline("circle").unwrap();

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
                if texture.dimensions.0 == 0 || texture.dimensions.1 == 0 {
                    warn!("Texture dimension is zero");
                }
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

                let descriptor_allocator = cache.descriptor_allocator.as_ref().unwrap();
                let pipeline = cache.get_pipeline("circle").unwrap();

                let layout = pipeline.layout().set_layouts().get(0).unwrap().clone();
                if layout.bindings().is_empty() {
                    warn!("Pipeline 'image' has no bindings. Did you forget to compile shaders?");
                }

                let image = Image::new(
                    cache.memory_allocator.clone().unwrap(),
                    ImageCreateInfo {
                        image_type: vulkano::image::ImageType::Dim2d,
                        format: vulkano::format::Format::R8G8B8A8_UNORM,
                        extent: [texture.dimensions.0, texture.dimensions.1, 1],
                        usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                        ..Default::default()
                    },
                    AllocationCreateInfo::default(),
                )
                .unwrap();
                let image_view = ImageView::new_default(image).unwrap();

                let descriptor_set = DescriptorSet::new(
                    descriptor_allocator.clone(),
                    layout,
                    [
                        WriteDescriptorSet::sampler(0, cache.sampler.clone()),
                        WriteDescriptorSet::image_view(1, image_view),
                    ],
                    [],
                )
                .unwrap();

                (verts, Some(descriptor_set))
            }
        }
    }
}
