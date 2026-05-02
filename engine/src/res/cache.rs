use std::{collections::HashMap, sync::{Arc, RwLock}};

use vulkano::pipeline::GraphicsPipeline;

pub struct Cache {
    pipelines: RwLock<HashMap<&'static str, Arc<GraphicsPipeline>>
>
}

impl Cache {
    pub fn new() -> Self {
        Self {
            pipelines: RwLock::new(HashMap::new())
        }
    }
}

pub(crate) trait PipelineHandle {
    fn get_pipeline(&self,key: &'static str) -> Option<Arc<GraphicsPipeline>>;
    fn insert_pipeline(&mut self, key: &'static str, pipeline: Arc<GraphicsPipeline>) -> Arc<GraphicsPipeline>;
}

impl PipelineHandle for Cache {
    fn get_pipeline(&self,key: &'static str) -> Option<Arc<GraphicsPipeline>> {
        self.pipelines.read().unwrap().get(key).cloned()
    }

    fn insert_pipeline(&mut self, key: &'static str, pipeline: Arc<GraphicsPipeline>) -> Arc<GraphicsPipeline> {
        self.pipelines.write().unwrap().insert(key, pipeline).unwrap()
    }
}
