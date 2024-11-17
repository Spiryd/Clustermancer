use std::str;

use rand::prelude::*;
use rand_pcg::Pcg64;

type Point = Vec<f64>;

struct StreamStorage {
    storage: Vec<Vec<(Point, usize)>>,
}

impl StreamStorage {
    fn new() -> StreamStorage {
        StreamStorage {
            storage: Vec::new(),
        }
    }
    fn add(&mut self, data: Vec<Point>, weights: Vec<usize>) {
        todo!()
    }
}

pub struct Stream {
    k: usize,
    buffer: Vec<Vec<f64>>,
    storage: StreamStorage,
    rng: Pcg64,
}

impl Stream {
    pub fn new(k: usize) -> Stream {
        Stream {
            k,
            buffer: Vec::with_capacity(2 * k),
            storage: StreamStorage::new(),
            rng: Pcg64::seed_from_u64(0),
        }
    }

    fn min_distance(point: &[f64], centers: &[Vec<f64>]) -> f64 {
        centers
            .iter()
            .map(|center| {
                point
                    .iter()
                    .zip(center.iter())
                    .map(|(x, y)| (x - y).powi(2))
                    .sum::<f64>()
                    .sqrt()
            })
            .fold(f64::MAX, |a, b| a.min(b))
    }

    fn find_nearest_center(point: &[f64], centers: &[Vec<f64>]) -> Vec<f64> {
        centers
            .iter()
            .min_by(|a, b| {
                let dist_a = point
                    .iter()
                    .zip(a.iter())
                    .map(|(x, y)| (x - y).powi(2))
                    .sum::<f64>()
                    .sqrt();
                let dist_b = point
                    .iter()
                    .zip(b.iter())
                    .map(|(x, y)| (x - y).powi(2))
                    .sum::<f64>()
                    .sqrt();
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .unwrap()
            .clone()
    }

    fn bicriteria_approximation(
        &mut self,
        s: Vec<Vec<f64>>,
        k: usize,
    ) -> (Vec<Vec<f64>>, Vec<usize>) {
        let k_prime = 2 * k;
        let mut centers: Vec<Vec<f64>> = Vec::new();
        centers.push(s[self.rng.gen_range(0..s.len())].clone());

        while centers.len() < k_prime {
            let distances: Vec<f64> = s
                .iter()
                .map(|point| Self::min_distance(point, &centers))
                .collect();
            let max_distance_index = distances
                .iter()
                .cloned()
                .enumerate()
                .fold((0, f64::MIN), |(max_idx, max_val), (idx, val)| {
                    if val > max_val {
                        (idx, val)
                    } else {
                        (max_idx, max_val)
                    }
                })
                .0;
            centers.push(s[max_distance_index].clone());
        }

        let mut clusters: Vec<Vec<Vec<f64>>> = vec![Vec::new(); k_prime];
        for point in s.iter() {
            let nearest_center = Self::find_nearest_center(point, &centers);
            let center_index = centers.iter().position(|c| *c == nearest_center).unwrap();
            clusters[center_index].push(point.clone());
        }

        let weights: Vec<usize> = clusters.iter().map(|cluster| cluster.len()).collect();
        (centers, weights)
    }

    pub fn insert(&mut self, data: Vec<f64>) {
        self.buffer.push(data);
        if self.buffer.len() >= 4 * 2 * self.k {
            let (centers, weights) = self.bicriteria_approximation(self.buffer.clone(), self.k);
            self.storage.add(centers, weights);
            self.buffer.clear();
        }
    }
}

impl super::DataStreamClusteringAlgorithm for Stream {
    fn insert(&mut self, data: Vec<f64>) {
        self.insert(data);
    }

    fn name(&self) -> String {
        "STREAM".to_string()
    }
}
