use std::path::Path;

use fontdue::layout::{Layout, TextStyle};

use crate::res::assets::Storage;

/// Stores the necessary parameters for fonts
pub struct Font {
    /// Character size
    size: f32,
    fonts: Vec<fontdue::Font>,
}

impl Font {
    /// Returns [`Font`] structure
    pub fn new(
        storage: Storage,
        size: f32,
        family: String,
        font_scale: Option<f32>,
    ) -> Self {
        let path_to_font = Path::new(&family);
        let font_in_bytes = storage.load(&path_to_font).unwrap();
        let font = fontdue::Font::from_bytes(
            font_in_bytes,
            fontdue::FontSettings {
                scale: font_scale.unwrap_or(fontdue::FontSettings::default().scale),
                ..Default::default()
            },
        )
        .unwrap();
        let mut fonts: Vec<fontdue::Font> = Vec::with_capacity(1);
        fonts.push(font);

        Self {
            size,
            fonts,
        }
    }

    pub fn get_glyphs(&self, layout: &mut Layout, text: String) {
        // TODO: Вынести все шрифты в отдельный массив в другой структуре и просто брать слайс с
        // индексом на шрифт
        layout.append(self.fonts.as_ref(), &TextStyle::new(&text, self.size, 0));
    }
}
