//! Image processing and creation
use image::GenericImageView;
use rust_embed::Embed;
use tracing::info;

#[derive(Embed)]
#[folder = "assets/"]
pub(crate) struct Asset; // TODO: Binary file large

/// Texture image struct which storage image in vector bytes and metadata
#[derive(Clone)]
pub struct Texture {
    #[allow(dead_code)]
    pub(crate) image: Vec<u8>,
    pub dimensions: (u32, u32),
}

impl Texture {
    /// Retrives texture from image file
    /// # Returns
    /// Texture struct
    pub fn from_file(path: &str) -> Option<Self> {
        match image::open(path).ok() {
            Some(img) => {
                info!("File import was successful");
                Some(Self {
                    image: img.clone().into_rgba8().into_raw().to_vec(),
                    dimensions: img.dimensions(),
                })
            }
            None => {
                info!("File {} not found", path);
                None
            }
        }
    }

    /// Retrives texture from internal asset directory
    /// # Returns
    /// Texture struct
    pub fn from_internal_assets(filename: &str) -> Option<Self> {
        match Asset::get(filename) {
            Some(f) => {
                info!("File from internal storage import was successful");
                let img = image::load_from_memory(f.data.as_ref()).unwrap();
                let dimensions = img.dimensions();
                Some(Self {
                    image: img.into_rgba8().as_raw().to_vec(),
                    dimensions,
                })
            }
            None => {
                info!("File {} not found in internal storage", filename);
                None
            }
        }
    }
}
