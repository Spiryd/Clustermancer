use super::Sampler;
use crate::algorithms::DataStreamClusteringAlgorithm;

pub struct DynamicSampler {
    algorithm: Box<dyn DataStreamClusteringAlgorithm>,
}

impl DynamicSampler {
    pub fn new(algorithm: Box<dyn DataStreamClusteringAlgorithm>) -> Self {
        Self { algorithm }
    }
}

impl Sampler for DynamicSampler {
    fn insert(&mut self, data: Vec<f64>) {
        
    }
    fn name(&self) -> String {
        "DynamicSampler".to_string()
    }
}
