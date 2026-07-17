use color::Rgba8;

use crate::Vector;

/// Text options
#[derive(Clone)]
pub struct SpriteTextCreateInfo {
    /// Text which will be draw on the screen
    pub text: String,
    ///
    pub scale: f32,
    /// Character size
    pub size: Vector,
    /// Text color
    pub color: Rgba8,
    /// Font index(temp)
    pub font: usize,
    /// Positional coordinates
    pub position: Vector,
}

impl SpriteTextCreateInfo {
    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn with_position(mut self, position: Vector) -> Self {
        self.position = position;
        self
    }

    pub fn with_color(mut self, color: Rgba8) -> Self {
        self.color = color;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn font_index(mut self, index: usize) -> Self {
        self.font = index;
        self
    }

    pub fn with_size(mut self, size: Vector) -> Self {
        self.size = size;
        self
    }
}

impl Default for SpriteTextCreateInfo {
    fn default() -> Self {
        Self {
            text: Default::default(),
            scale: 20.0,
            size: Vector::new(15.0, 7.5),
            color: Rgba8 {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
            font: 0,
            position: Default::default(),
        }
    }
}
