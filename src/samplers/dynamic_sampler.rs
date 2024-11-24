use super::Sampler;
use crate::algorithms::DataStreamClusteringAlgorithm;

const K: usize = 5;
const MAX_ITERATIONS: usize = 10_000;

pub fn kmeans(data: &[Vec<f64>], k: usize, max_iterations: usize) -> Vec<usize> {
    let mut centroids = initialize_centroids(data, k);
    let mut assignments = vec![0; data.len()];

    for _ in 0..max_iterations {
        // Assign clusters
        for (i, point) in data.iter().enumerate() {
            assignments[i] = closest_centroid(point, &centroids);
        }

        // Update centroids
        let mut new_centroids = vec![vec![0.0; data[0].len()]; k];
        let mut counts = vec![0; k];

        for (assignment, point) in assignments.iter().zip(data.iter()) {
            for (j, value) in point.iter().enumerate() {
                new_centroids[*assignment][j] += value;
            }
            counts[*assignment] += 1;
        }

        for (centroid, count) in new_centroids.iter_mut().zip(counts.iter()) {
            if *count > 0 {
                for value in centroid.iter_mut() {
                    *value /= *count as f64;
                }
            }
        }

        centroids = new_centroids;
    }

    assignments
}

fn initialize_centroids(data: &[Vec<f64>], k: usize) -> Vec<Vec<f64>> {
    data.iter().take(k).cloned().collect()
}

fn closest_centroid(point: &[f64], centroids: &[Vec<f64>]) -> usize {
    centroids
        .iter()
        .enumerate()
        .map(|(i, centroid)| (i, euclidean_distance(point, centroid)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap()
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub struct DynamicSampler {
    algorithm: Box<dyn DataStreamClusteringAlgorithm>,
    initial_buffer: Vec<Vec<f64>>,
    initialised: bool,
}

impl DynamicSampler {
    pub fn new(algorithm: Box<dyn DataStreamClusteringAlgorithm>) -> Self {
        Self {
            algorithm,
            initial_buffer: Vec::new(),
            initialised: false,
        }
    }
}

impl Sampler for DynamicSampler {
    fn insert(&mut self, data: Vec<f64>) {
        if !self.initialised {
            self.algorithm.insert(data.clone());
            self.initial_buffer.push(data);
            if self.initial_buffer.len() >= 1000 {
                let assignments = kmeans(&self.initial_buffer, K, MAX_ITERATIONS);

                self.initialised = true;
            }
        } else {
            // sample with probability tied to closenes to centroids
            self.algorithm.insert(data);
        }
    }
    fn name(&self) -> String {
        "DynamicSampler".to_string()
    }
}
