pub mod birch;
pub mod clustream;
pub mod denstream;

pub struct ClusteringElement {
    pub center: Vec<f64>,
    pub radius: f64,
    pub cluster: usize,
}

pub trait DataStreamClusteringAlgorithm {
    fn insert(&mut self, data: Vec<f64>);
    fn name(&self) -> String;
    fn clusters(&self) -> Vec<ClusteringElement>;
}
