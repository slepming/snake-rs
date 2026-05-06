use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use vulkano::{descriptor_set::DescriptorSet, pipeline::GraphicsPipeline};

pub struct Cache {
    pipelines: RwLock<HashMap<&'static str, Arc<GraphicsPipeline>>>,
    descriptors: RwLock<HashMap<&'static str, Arc<DescriptorSet>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            pipelines: RwLock::new(HashMap::new()),
            descriptors: RwLock::new(HashMap::new()),
        }
    }
}

pub(crate) trait PipelineHandle {
    fn get_pipeline(&self, key: &'static str) -> Option<Arc<GraphicsPipeline>>;
    fn insert_pipeline(
        &self,
        key: &'static str,
        pipeline: Arc<GraphicsPipeline>,
    ) -> Option<Arc<GraphicsPipeline>>;
}

impl PipelineHandle for Cache {
    fn get_pipeline(&self, key: &'static str) -> Option<Arc<GraphicsPipeline>> {
        self.pipelines.read().unwrap().get(key).cloned()
    }

    fn insert_pipeline(
        &self,
        key: &'static str,
        pipeline: Arc<GraphicsPipeline>,
    ) -> Option<Arc<GraphicsPipeline>> {
        self.pipelines.write().unwrap().insert(key, pipeline)
    }
}
