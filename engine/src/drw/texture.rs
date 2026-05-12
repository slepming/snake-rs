use anyhow::Ok;
use image::{DynamicImage, GenericImageView, ImageResult};

#[derive(Clone)]
pub struct Texture {
    image: Vec<u8>,
}

impl Texture
{
    //pub fn from_file(path: &str) -> Result<Self, image::ImageError> {
    //    let img = image::open(path)?;
    //}
}
