#[derive(Debug, Clone)]
struct MicroCluster {
    cf2x: Vec<f64>,
    cf1x: Vec<f64>,
    cf2t: f64,
    cf1t: f64,
    n: usize,
}

impl MicroCluster {
    fn new(instance: Vec<f64>, time_stamp: usize) -> Self {
        MicroCluster { 
            cf2x: instance.iter().map(|x| x * x).collect(),
            cf1x: instance.clone(),
            cf2t: (time_stamp * time_stamp) as f64,
            cf1t: time_stamp as f64,
            n: 1,
        }
    }
}

pub struct CluStream{}

impl CluStream {
    pub fn new() -> Self {
        CluStream{}
    }

    pub fn insert(&mut self, instance: Vec<f64>) {
        todo!()
    }

    pub fn offline_refinement(&self) {
        todo!()
    }
}
