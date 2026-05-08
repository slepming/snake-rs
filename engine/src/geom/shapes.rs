use std::sync::Arc;
use strum::IntoStaticStr;
use tracing::warn;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter};
use vulkano::descriptor_set::{DescriptorSet, WriteDescriptorSet};
use vulkano::pipeline::Pipeline;

use crate::MyVertex;
use crate::res::cache::{Cache, PipelineHandle};

#[derive(vulkano::buffer::BufferContents, Clone, Copy)]
#[repr(C)]
pub struct CircleData {
    pub radius: f32,
    pub thickness: f32,
}

#[derive(IntoStaticStr, Clone, Copy)]
pub enum Shapes {
    Square(),
    Circle(),
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
            Shapes::Square() => {
                let verts = vec![
                    MyVertex { position: [-1.0, -1.0] },
                    MyVertex { position: [1.0, -1.0] },
                    MyVertex { position: [1.0, 1.0] },
                    MyVertex { position: [-1.0, 1.0] },
                ];
                (verts, None)
            }
            Shapes::Circle() => {
                let verts = vec![
                    MyVertex { position: [-1.0, -1.0] },
                    MyVertex { position: [1.0, -1.0] },
                    MyVertex { position: [1.0, 1.0] },
                    MyVertex { position: [-1.0, 1.0] },
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
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    CircleData { radius: 0.05, thickness: 0.001 },
                ).unwrap();

                let layout = pipeline.layout().set_layouts().get(0).unwrap().clone();
                if layout.bindings().is_empty() {
                    warn!("Pipeline 'circle' has no bindings. Did you forget to compile shaders?");
                }
                
                let descriptor_set = DescriptorSet::new(
                    descriptor_allocator.clone(),
                    layout,
                    [WriteDescriptorSet::buffer(0, buffer)],
                    [],
                ).unwrap();

                (verts, Some(descriptor_set))
            }
        }
    }
}
