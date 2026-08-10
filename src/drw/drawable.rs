//! Managing Drawable states

use crate::{Vector, ecs::tables::ClassInfo, geom::shapes::Shapes};

use color::Rgba8;

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
