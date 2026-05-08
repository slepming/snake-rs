use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use vulkano::{
    descriptor_set::{DescriptorSet, allocator::StandardDescriptorSetAllocator},
    memory::allocator::StandardMemoryAllocator,
    pipeline::GraphicsPipeline,
};

pub struct Cache {
    pipelines: RwLock<HashMap<&'static str, Arc<GraphicsPipeline>>>,
    descriptors: RwLock<HashMap<String, Arc<DescriptorSet>>>,
    pub memory_allocator: Option<Arc<StandardMemoryAllocator>>,
    pub descriptor_allocator: Option<Arc<StandardDescriptorSetAllocator>>,
}

impl Cache {
    pub fn new(
        memory_allocator: Option<Arc<StandardMemoryAllocator>>,
        descriptor_allocator: Option<Arc<StandardDescriptorSetAllocator>>,
    ) -> Self {
        Self {
            pipelines: RwLock::new(HashMap::new()),
            descriptors: RwLock::new(HashMap::new()),
            memory_allocator,
            descriptor_allocator,
        }
    }
}

pub(crate) trait PipelineHandle {
    fn get_pipeline(&self, key: &str) -> Option<Arc<GraphicsPipeline>>;
    fn insert_pipeline(&self, key: &'static str, pipeline: Arc<GraphicsPipeline>) -> &Self;
}

impl PipelineHandle for Cache {
    fn get_pipeline(&self, key: &str) -> Option<Arc<GraphicsPipeline>> {
        self.pipelines.read().unwrap().get(key).cloned()
    }

    fn insert_pipeline(&self, key: &'static str, pipeline: Arc<GraphicsPipeline>) -> &Self {
        self.pipelines.write().unwrap().insert(key, pipeline);
        self
    }
}

pub(crate) trait DescriptorHandle {
    fn get_descriptor(&self, key: &str) -> Option<Arc<DescriptorSet>>;
    fn insert_descriptor_set(&self, key: String, descriptor_set: Arc<DescriptorSet>) -> &Self;
}

impl DescriptorHandle for Cache {
    fn get_descriptor(&self, key: &str) -> Option<Arc<DescriptorSet>> {
        self.descriptors.read().unwrap().get(key).cloned()
    }

    fn insert_descriptor_set(&self, key: String, descriptor: Arc<DescriptorSet>) -> &Self {
        self.descriptors.write().unwrap().insert(key, descriptor);
        self
    }
}
