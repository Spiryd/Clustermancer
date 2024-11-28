use super::Sampler;
use crate::algorithms::DataStreamClusteringAlgorithm;
use rand::prelude::*;
use rand_pcg::Pcg64;

pub struct StaticSampler {
    algorithm: Box<dyn DataStreamClusteringAlgorithm>,
    odds: f64,
    rng: Pcg64,
}

impl StaticSampler {
    pub fn new(algorithm: Box<dyn DataStreamClusteringAlgorithm>, odds: f64) -> Self {
        Self {
            algorithm,
            odds,
            rng: Pcg64::from_entropy(),
        }
    }
}

impl Sampler for StaticSampler {
    fn insert(&mut self, data: Vec<f64>) {
        if self.rng.gen_bool(self.odds) {
            self.algorithm.insert(data);
        }
    }
    fn name(&self) -> String {
        format!("(StaticSampler({}), {})", self.odds, self.algorithm.name())
    }
    fn clusters(&self) -> Vec<crate::algorithms::ClusteringElement> {
        self.algorithm.clusters()
    }
}
