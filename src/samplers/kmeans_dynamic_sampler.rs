use super::Sampler;
use crate::algorithms::DataStreamClusteringAlgorithm;
use itertools::Itertools;
use rand::prelude::*;
use rand_pcg::Pcg64;

const MAX_ITERATIONS: usize = 10_000;
const DELTA: usize = 1_000;
const ALPHA: f64 = 1.0;
const BETA: f64 = 0.5;
const LAMBDA: f64 = 0.001;

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

struct ClusterCharacteristics {
    centroid: Vec<f64>,
    radius: f64,
    _count: usize,
}

pub struct KMeansDynamicSampler {
    algorithm: Box<dyn DataStreamClusteringAlgorithm>,
    assignments: Vec<ClusterCharacteristics>,
    max_distance: f64,
    rng: Pcg64,
    initial_buffer: Vec<Vec<f64>>,
    initialised: bool,
    k: usize,
}

impl KMeansDynamicSampler {
    pub fn new(algorithm: Box<dyn DataStreamClusteringAlgorithm>, k: usize) -> Self {
        Self {
            algorithm,
            assignments: Vec::new(),
            max_distance: f64::MAX,
            rng: Pcg64::from_entropy(),
            initial_buffer: Vec::new(),
            initialised: false,
            k,
        }
    }
}

impl Sampler for KMeansDynamicSampler {
    fn insert(&mut self, data: Vec<f64>) {
        if !self.initialised {
            self.algorithm.insert(data.clone());
            self.initial_buffer.push(data);
            if self.initial_buffer.len() >= DELTA {
                let kmeans_result = kmeans(&self.initial_buffer, self.k, MAX_ITERATIONS);
                let mut clusters = vec![Vec::new(); self.k];
                for (assignment, point) in kmeans_result.iter().zip(self.initial_buffer.iter()) {
                    clusters[*assignment].push(point.clone());
                }
                for cluster in clusters {
                    let centroid: Vec<f64> = cluster
                        .iter()
                        .fold(vec![0.0; cluster[0].len()], |mut acc, e| {
                            for (i, val) in e.iter().enumerate() {
                                acc[i] += val;
                            }
                            acc
                        })
                        .iter()
                        .map(|e| e / cluster.len() as f64)
                        .collect();
                    let radius = (cluster
                        .iter()
                        .map(|e| {
                            e.iter()
                                .zip(centroid.iter())
                                .map(|(a, b)| (a - b).powi(2))
                                .sum::<f64>()
                        })
                        .sum::<f64>()
                        / cluster.len() as f64)
                        .sqrt();
                    self.assignments.push(ClusterCharacteristics {
                        centroid,
                        radius,
                        _count: cluster.len(),
                    });
                }
                self.max_distance = 
                self.initial_buffer.iter().combinations(2).map(|pair| {
                    euclidean_distance(pair[0], pair[1])
                }).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
                self.initialised = true;
                self.initial_buffer.clear();
            }
        } else {
            // sample with probability tied to closenes to centroids
            let (_closest_cluster, min_distance) = self
                .assignments
                .iter()
                .map(|cluster| {
                    let distance = euclidean_distance(&data, &cluster.centroid) - cluster.radius;
                    (cluster, distance)
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();
            let probability: f64;
            if min_distance <= 0.0 {
                probability = BETA;
            } else if min_distance >= self.max_distance {
                probability = LAMBDA;
                self.max_distance = min_distance;
            } else {
                probability = ALPHA * (min_distance / self.max_distance);
            }
            if self.rng.gen_bool(probability) {
                self.algorithm.insert(data);
            }
        }
    }
    fn name(&self) -> String {
        format!("(KMeansDynamicSampler, {})", self.algorithm.name())
    }
    fn clusters(&self) -> Vec<crate::algorithms::ClusteringElement> {
        self.algorithm.clusters()
    }
}
