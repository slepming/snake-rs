//! Image processing and creation
use std::fmt::Debug;

use image::GenericImageView;
use tracing::{debug, info};

use crate::{geom::dimension::Dimension, res::assets::Asset};

/// Texture image struct which storage image in vector bytes and metadata
#[derive(Clone)]
pub struct Texture {
    #[allow(dead_code)]
    pub(crate) image: Vec<u8>,
    pub dimension: Dimension,
}

impl Texture {
    pub fn new(stream: Vec<u8>, dimension: Dimension) -> Self {
        Self {
            image: stream,
            dimension,
        }
    }
    /// Retrives texture from image file
    /// # Returns
    /// Texture struct
    pub fn from_file(path: &str) -> Option<Self> {
        match image::open(path).ok() {
            Some(img) => {
                info!("File import was successful");
                Some(Self {
                    image: img.clone().into_rgba8().into_raw().to_vec(),
                    dimension: Dimension::from(img.dimensions()),
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
                let dimension = img.dimensions();
                Some(Self {
                    image: img.into_rgba8().as_raw().to_vec(),
                    dimension: Dimension::from(dimension),
                })
            }
            None => {
                info!("File {} not found in internal storage", filename);
                None
            }
        }
    }

    pub fn from_slice(bytes: &[u8]) -> Option<Self> {
        if bytes.len() == 0 {
            return None;
        }

        debug!("Load texture from byte slice");
        let img = image::load_from_memory(bytes).unwrap();
        let dimension = img.dimensions();

        Some(Self {
            image: img.into_rgba8().as_raw().to_vec(),
            dimension: Dimension::from(dimension),
        })
    }
}

impl Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture").field("image_len", &self.image.len()).field("dimension", &self.dimension).finish()
    }
}
