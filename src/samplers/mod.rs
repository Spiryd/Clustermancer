pub mod kmeans_dynamic_sampler;
pub mod static_sampler;

use crate::algorithms::ClusteringElement;

pub trait Sampler {
    fn insert(&mut self, data: Vec<f64>);
    fn name(&self) -> String;
    fn clusters(&self) -> Vec<ClusteringElement>;
}
