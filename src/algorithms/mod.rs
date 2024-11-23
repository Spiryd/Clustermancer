pub mod birch;
pub mod clustream;
pub mod denstream;

pub trait DataStreamClusteringAlgorithm {
    fn insert(&mut self, data: Vec<f64>);
    fn name(&self) -> String;
}
