pub mod birch;
pub mod clustream;
pub mod denstream;

pub struct ClusteringElement {
    pub center: Vec<f64>,
    pub radius: f64,
    pub cluster: usize,
}

impl ClusteringElement {
    pub fn distance(&self, other: &Vec<f64>) -> f64 {
        self.center
            .iter()
            .zip(other.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

pub fn ssq(clusters: &Vec<ClusteringElement>) -> f64 {
    let k = clusters
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, elem| {
            acc.entry(elem.cluster).or_insert_with(Vec::new).push(elem);
            acc
        });
    let mut ssq = 0.;
    for k_i in k.values() {
        let center = k_i.iter().fold(vec![0.0; k_i[0].center.len()], |acc, elem| {
            acc.iter()
                .zip(elem.center.iter())
                .map(|(a, b)| a + b)
                .collect()
        }).iter().map(|elem| elem / k_i.len() as f64).collect::<Vec<f64>>();
        ssq += k_i.iter().map(|elem| elem.distance(&center).powi(2)).sum::<f64>();
    }
    ssq
}

pub trait DataStreamClusteringAlgorithm {
    fn insert(&mut self, data: Vec<f64>);
    fn name(&self) -> String;
    fn clusters(&self) -> Vec<ClusteringElement>;
}
