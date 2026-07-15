//! Managing Drawable states

use std::sync::Arc;

use vulkano::{
    descriptor_set::allocator::DescriptorSetAllocator, image::sampler::Sampler,
    memory::allocator::MemoryAllocator,
};

use crate::{
    MyVertex, Vector,
    geom::{matrix::Transform, shapes::Shapes},
    mv::transform::{HasTransform, Positioned},
    res::cache::{DescriptorSetCache, PipelineCache},
};

use color::Rgba8;
use vulkano::descriptor_set::DescriptorSet;

/// The main element that is rendered by the Vulkan
#[derive(PartialEq, Debug)]
pub struct Drawable {
    transform: Transform,
    color: Rgba8,
    pub(crate) render: DrawableRenderContext,
}

/// Pipeline id structure
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PipelineID {
    /// String ID to search pipelines
    pub id: String,
}

/// Descriptor set id structure
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DescriptorID {
    /// String ID to search descriptor sets
    pub id: String,
}

#[derive(PartialEq, Debug)]
pub(crate) struct DrawableRenderContext {
    /// Memory descriptor key(id). Drawable doesn't know anything about the descriptor
    pub(crate) descriptor_id: DescriptorID,
    /// Pipeline key(id). Drawable doesn't know anything about the pipeline
    pub(crate) pipeline_id: PipelineID,
    pub mesh: Mesh,
}

/// Information about the object to be drawn
#[derive(Debug, Clone)]
pub struct DrawableCreateInfo {
    /// Positional coordinates
    pub position: Vector,
    /// Drawable object size
    pub size: Vector,
    pub(crate) id: u32,
    /// Object color
    pub color: Rgba8,
}

impl DrawableCreateInfo {
    /// Sets the position of the object
    ///
    /// # Returns
    /// [`DrawableCreateInfo`]
    pub fn with_position(mut self, position: Vector) -> Self {
        self.position = position;
        self
    }

    /// Sets the size of the object
    ///
    /// # Returns
    /// [`DrawableCreateInfo`]
    pub fn with_size(mut self, size: Vector) -> Self {
        self.size = size;
        self
    }

    /// Sets id of the object
    ///
    /// # Returns
    /// [`DrawableCreateInfo`]
    pub(crate) fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Sets the color for the object
    ///
    /// # Returns
    /// [`DrawableCreateInfo`]
    pub fn with_color(mut self, color: Rgba8) -> Self {
        self.color = color;
        self
    }
}

impl Default for DrawableCreateInfo {
    fn default() -> Self {
        Self {
            position: Default::default(),
            size: Default::default(),
            id: Default::default(),
            color: Rgba8 {
                r: 255,
                g: 255,
                b: 255,
                a: 1,
            },
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    vertex: Vec<MyVertex>,
    /// ID need for find matrix in buffer
    id: u32,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
    }
}

pub trait DrawableGPU {
    #[allow(dead_code)]
    fn set_vertex(&mut self, vertex: Vec<MyVertex>);
    #[allow(dead_code)]
    fn vertex_clone(&self) -> Vec<MyVertex>;
    fn vertex(&self) -> &Vec<MyVertex>;
    /// # Returns
    /// Colour for shader
    fn colour(&self) -> &Rgba8;
}

pub trait DrawableComponent: DrawableGPU {
    /// # Returns
    /// [`Transform`] pointer
    fn transform(&self) -> &Transform;
    /// # Returns
    /// [`Transform`] mutable pointer
    fn transform_mut(&mut self) -> &mut Transform;
    /// # Returns
    /// [`Transform`] clone
    fn transform_clone(&self) -> Transform;
    /// Sets transform matrix
    fn set_transform(&mut self, transform: Transform);
    /// # Returns
    /// Reference to drawable
    fn drawable(&self) -> &Drawable;
    /// # Returns
    /// Mutable drawable
    fn drawable_mut(&mut self) -> &mut Drawable;
    /// Returns drawable size
    fn size(&self) -> Vector;
}

impl Mesh {
    pub fn new(ver: Vec<MyVertex>, id: u32) -> Self {
        Mesh { vertex: ver, id }
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }
}

impl Drawable {
    pub fn new(
        drawable_info: DrawableCreateInfo,
        pipeline_id: PipelineID,
        descriptor_id: DescriptorID,
        vertex: Vec<MyVertex>,
    ) -> Self {
        let pos = drawable_info.position;
        let transform = Transform([
            [drawable_info.size[0], 0.0, 0.0, 0.0],
            [0.0, drawable_info.size[1], 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [pos[0], pos[1], 0.0, 1.0],
        ]);

        let drawable = Drawable {
            color: drawable_info.color,
            transform,
            render: DrawableRenderContext {
                descriptor_id,
                pipeline_id,
                mesh: Mesh::new(vertex, drawable_info.id),
            },
        };

        drawable
    }

    /// Creates allocations, pipeline descriptors for drawable and calls [`Drawable::new_with_color`]
    ///
    /// # Returns
    /// ([`Drawable`], [`vulkano::descriptor_set::DescriptorSet`])
    pub fn from_shape(
        shape: Shapes,
        drw: DrawableCreateInfo,
        mem_alloc: Arc<dyn MemoryAllocator>,
        desc_alloc: Arc<dyn DescriptorSetAllocator>,
        pipeline_cache: Arc<PipelineCache>,
        desc_cache: Arc<DescriptorSetCache>,
        sampler: Option<Arc<Sampler>>,
    ) -> (Self, Option<Arc<DescriptorSet>>) {
        let key_raw: &'static str = shape.clone().into();
        let key = key_raw.to_string().to_lowercase();
        let pipeline_id = PipelineID { id: key.clone() };

        let descriptor_id = DescriptorID {
            id: drw.id.to_string(),
        };

        let (vertex, desc) = shape.get_vertex_and_descriptor(
            pipeline_id.clone(),
            descriptor_id.clone(),
            mem_alloc,
            desc_alloc,
            desc_cache,
            pipeline_cache,
            sampler,
        );
        (Drawable::new(drw, pipeline_id, descriptor_id, vertex), desc)
    }
}

impl DrawableGPU for Drawable {
    fn set_vertex(&mut self, vertex: Vec<MyVertex>) {
        self.render.mesh.vertex = vertex;
    }

    fn vertex_clone(&self) -> Vec<MyVertex> {
        self.render.mesh.vertex.clone()
    }

    fn vertex(&self) -> &Vec<MyVertex> {
        &self.render.mesh.vertex
    }

    fn colour(&self) -> &Rgba8 {
        &self.color
    }
}

impl DrawableComponent for Drawable {
    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn transform_clone(&self) -> Transform {
        self.transform.clone()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    fn drawable(&self) -> &Drawable {
        &self
    }

    fn drawable_mut(&mut self) -> &mut Drawable {
        self
    }

    fn size(&self) -> Vector {
        Vector::new(self.transform.0[0][0], self.transform.0[1][1]) // 0 0 -> width; 1 1 -> height
    }
}

impl Positioned for Drawable {
    fn position(&self) -> Vector {
        let transform = self.transform.matrix();

        Vector::new(transform[3][0], transform[3][1])
    }

    fn set_position(&mut self, vec: Vector) {
        let current_transform = self.transform_mut();
        let current_matrix = current_transform.matrix_mut();

        current_matrix[0][0] = vec.x;
        current_matrix[1][1] = vec.y;
    }
}
