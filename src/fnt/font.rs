use std::path::Path;

use crate::res::assets::Storage;

pub struct Font {
    size: u32,
    width: u32,
    family: String,
    italic: bool,
    bold: bool,
    font: fontdue::Font,
}

impl Font {
    pub fn new(
        storage: Storage,
        size: u32,
        width: u32,
        family: String,
        font_scale: Option<f32>,
        italic: Option<bool>,
        bold: Option<bool>,
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
        Self {
            size,
            width,
            family,
            italic: italic.unwrap_or(false),
            bold: bold.unwrap_or(false),
            font,
        }
    }
}
