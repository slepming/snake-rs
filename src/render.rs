use std::sync::Arc;

use vulkano::{
    buffer::Subbuffer,
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, RenderPass},
    swapchain::Swapchain,
    sync::GpuFuture,
};
use winit::window::Window;

use crate::{MyVertex, geom::matrix::Transform};

pub(crate) struct RenderContext {
    pub window: Arc<Window>,
    pub swapchain: Arc<Swapchain>,
    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub viewport: Viewport,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

/// Used for drawable calculations
pub(crate) struct MeshBuffers(pub Subbuffer<[MyVertex]>, pub Vec<Transform>, pub Vec<u32>);
