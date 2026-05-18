use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use vulkano::{
    descriptor_set::{DescriptorSet, allocator::StandardDescriptorSetAllocator},
    pipeline::GraphicsPipeline,
};

pub trait CacheProvider<A: Sized>
{
    fn get(&self, key: &str) -> Option<A>;
    fn insert(&self, value: (String, A)) -> Option<A>;
}

pub struct DescriptorSetCache
{
    descriptors: RwLock<HashMap<String, Arc<DescriptorSet>>>
}

impl DescriptorSetCache
{
    pub fn new(descriptors: HashMap<String, Arc<DescriptorSet>>) -> Self
    {
        Self { descriptors: RwLock::new(descriptors) }
    }

}

impl CacheProvider<Arc<DescriptorSet>> for DescriptorSetCache {
    fn get(&self, key: &str) -> Option<Arc<DescriptorSet>> {
        self.descriptors.read().unwrap().get(key).cloned()
    }

    fn insert(&self, descriptor: (String, Arc<DescriptorSet>)) -> Option<Arc<DescriptorSet>> {
        self.descriptors.write().unwrap().insert(descriptor.0, descriptor.1)
    }
}

impl Default for DescriptorSetCache {
    fn default() -> Self {
        Self { descriptors: RwLock::new(HashMap::default()) }
    }
}

pub struct PipelineCache
{
    pipelines: RwLock<HashMap<String, Arc<GraphicsPipeline>>>,
}

impl PipelineCache 
{
    pub fn new(pipelines: HashMap<String, Arc<GraphicsPipeline>>) -> Self {
        Self { pipelines: RwLock::new(pipelines)}
    }
}

impl Default for PipelineCache
{
    fn default() -> Self {
        Self { pipelines: RwLock::new(HashMap::default()) }
    }
}

impl CacheProvider<Arc<GraphicsPipeline>> for PipelineCache {
    fn get(&self, pipeline_key: &str) -> Option<Arc<GraphicsPipeline>> {
        self.pipelines.read().unwrap().get(pipeline_key).cloned()
    }

    fn insert(&self, pipeline: (String, Arc<GraphicsPipeline>)) -> Option<Arc<GraphicsPipeline>> {
        self.pipelines.write().unwrap().insert(pipeline.0, pipeline.1)
    }
}
