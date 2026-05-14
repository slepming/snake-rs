pub mod image_vs {
    vulkano_shaders::shader! {
        bytes: "shaders/image.vert.spv"
    }
}

pub mod image_fs {
    vulkano_shaders::shader! {
        bytes: "shaders/image.frag.spv"
    }
}
