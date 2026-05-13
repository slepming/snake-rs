use image::GenericImageView;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub(crate) struct Asset;

/// Texture image struct which storage image in vector bytes and metadata
#[derive(Clone)]
pub struct Texture {
    image: Vec<u8>,
    dimensions: (u32, u32)
}

impl Texture
{
    /// Retrives texture from disk file
    /// # Returns
    /// Texture struct
    pub fn from_file(path: &str) -> Option<Self> {
        match image::open(path).ok() {
            Some(img) => Some(Self { image: img.clone().into_bytes(), dimensions: img.dimensions() } ),
            None => None
        }
    }
}
