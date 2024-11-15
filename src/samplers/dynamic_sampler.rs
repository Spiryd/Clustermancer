use crate::algorithms::DataStreamClusteringAlgorithm;
use super::Sampler;

pub struct DynamicSampler {
    algorithm: Box<dyn DataStreamClusteringAlgorithm>,
    counter: usize,
}

impl DynamicSampler {
    pub fn new(algorithm: Box<dyn DataStreamClusteringAlgorithm>) -> Self {
        Self { algorithm, counter: 1 }
    }
}

impl Sampler for DynamicSampler {
    fn insert(&mut self, data: Vec<f64>) {
        todo!()
    }
    fn name(&self) -> String {
        format!("DynamicSampler")
    }
}