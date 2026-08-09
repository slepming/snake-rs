//! Managing Drawable states

use crate::{
    SnakeVertex, Vector,
    ecs::tables::ClassInfo,
    geom::{matrix::Transform, shapes::Shapes},
};

use color::Rgba8;

/// The main element that is rendered by the Vulkan
#[derive(PartialEq, Debug)]
pub struct Drawable {
    color: Rgba8,
    pub(crate) render: DrawableRenderContext,
}

/// Pipeline id structure
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PipelineID {
    /// String ID to search pipelines
    pub id: String,
}

impl From<Shapes> for PipelineID {
    fn from(value: Shapes) -> Self {
        Self {
            id: value.as_ref().to_lowercase().to_string(),
        }
    }
}

/// Descriptor set id structure
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DescriptorID {
    /// String ID to search descriptor sets
    pub id: String,
}

impl From<&ClassInfo> for DescriptorID {
    fn from(value: &ClassInfo) -> Self {
        Self {
            id: value.class_name.to_string(),
        }
    }
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

    #[allow(dead_code)]
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
                a: 255,
            },
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    vertex: &'static [SnakeVertex],
    /// ID need for find matrix in buffer
    id: u32,
}

impl PartialEq for Mesh {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
    }
}

pub trait DrawableGPU {
    fn vertex(&self) -> &'static [SnakeVertex];
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
    pub fn new(ver: &'static [SnakeVertex], id: u32) -> Self {
        Mesh { vertex: ver, id }
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }
}
