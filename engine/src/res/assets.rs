use std::{path::Path, sync::Arc};

use image::ImageReader;
use vulkano::{
    DeviceSize,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CopyBufferToImageInfo, PrimaryCommandBufferAbstract,
    },
    device::Queue,
    image::{Image, ImageCreateInfo, ImageUsage, view::ImageView},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    sync::GpuFuture,
};

use crate::{drw::texture::Texture, mem::engine_memory::EngineMemory};

pub struct AssetsManager {
    pub(crate) queue: Arc<Queue>,
    pub(crate) memory_allocs: Arc<EngineMemory>,
}

#[derive(Clone)]
pub struct TextureHandler {
    pub(crate) view: Arc<ImageView>,
}

impl AssetsManager {
    pub fn load(&self, file_name: &Path, internal: bool) -> TextureHandler {
        let texture: Texture = {
            if internal {
                Texture::from_internal_assets(file_name.to_str().unwrap()).unwrap()
            } else {
                Texture::from_file(file_name.to_str().unwrap()).unwrap()
            }
        };

        let mut uploads = AutoCommandBufferBuilder::primary(
            self.memory_allocs.command_buffer_allocator.clone(),
            self.queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let image_view: Arc<ImageView> = {
            let upload_buffer = Buffer::new_slice::<u8>(
                self.memory_allocs.memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::TRANSFER_SRC,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_HOST
                        | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                (texture.dimensions.0 * texture.dimensions.1 * 4) as DeviceSize,
            )
            .unwrap();

            upload_buffer
                .write()
                .unwrap()
                .copy_from_slice(&texture.image);

            let image = Image::new(
                self.memory_allocs.memory_allocator.clone(),
                ImageCreateInfo {
                    image_type: vulkano::image::ImageType::Dim2d,
                    format: vulkano::format::Format::R8G8B8A8_UNORM,
                    extent: [texture.dimensions.0, texture.dimensions.1, 1],
                    usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            )
            .unwrap();

            drop(texture);

            uploads
                .copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
                    upload_buffer,
                    image.clone(),
                ))
                .unwrap();
            ImageView::new_default(image).unwrap()
        };

        let _ = uploads
            .build()
            .unwrap()
            .execute(self.queue.clone())
            .unwrap()
            .flush();

        TextureHandler { view: image_view }
    }
}
