pub struct STREAM {}

impl STREAM {
    pub fn new() -> STREAM {
        STREAM {}
    }
}

impl super::DataStreamClusteringAlgorithm for STREAM {
    fn insert(&mut self, data: Vec<f64>) {
        print!("STREAM: {:?}", data);
    }
    fn name(&self) -> String {
        "STREAM".to_string()
    }
}
