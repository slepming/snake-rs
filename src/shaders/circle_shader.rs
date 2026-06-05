pub mod circle_vs {
    vulkano_shaders::shader! {
        bytes: "shaders/circle.vert.spv"
    }
}

pub mod circle_fs {
    vulkano_shaders::shader! {
        bytes: "shaders/circle.frag.spv"
    }
}
