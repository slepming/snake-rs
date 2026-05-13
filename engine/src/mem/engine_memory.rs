use std::sync::Arc;

use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator, device::Device,
    memory::allocator::StandardMemoryAllocator,
};

/// Engine memory allocators storage
pub(crate) struct EngineMemory {
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub memory_allocator: Arc<StandardMemoryAllocator>,
    pub descriptor_allocator: Arc<StandardDescriptorSetAllocator>,
}

impl EngineMemory {
    pub fn new(device: Arc<Device>) -> Self {
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        // Before we can start creating and recording command buffers, we need a way of allocating
        // them. Vulkano provides a command buffer allocator, which manages raw Vulkan command
        // pools underneath and provides a safe interface for them.
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));
        EngineMemory {
            descriptor_allocator: descriptor_set_allocator,
            command_buffer_allocator,
            memory_allocator,
        }
    }
}
