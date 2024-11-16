use crate::algorithms::DataStreamClusteringAlgorithm;
use super::Sampler;
use rand::prelude::*;
use rand_pcg::Pcg64;

pub struct UniformSampler {
    algorithm: Box<dyn DataStreamClusteringAlgorithm>,
    counter: usize,
    rng: Pcg64,
}

impl UniformSampler {
    pub fn new(algorithm: Box<dyn DataStreamClusteringAlgorithm>) -> Self {
        Self { algorithm, counter: 1, rng: Pcg64::from_entropy() }
    }
}

impl Sampler for UniformSampler {
    fn insert(&mut self, data: Vec<f64>) {
        if self.rng.gen_bool(1.0 / self.counter as f64) {
            println!("Inserting at: {:?}", self.counter);
            self.algorithm.insert(data);
        }
        self.counter += 1;
    }
    fn name(&self) -> String {
        "UniformSampler".to_string()
    }
}