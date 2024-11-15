pub mod uniform_sampler;
pub mod dynamic_sampler;

pub trait Sampler {
    fn insert(&mut self, data: Vec<f64>);
    fn name(&self) -> String;
}