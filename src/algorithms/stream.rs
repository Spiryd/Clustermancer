use rand::prelude::*;
use rand_pcg::Pcg64;

fn kmeans(instances: Vec<Vec<f64>>, k: usize, max_iterations: usize) -> Vec<usize> {
    let mut rng = Pcg64::from_entropy();
    let mut centroids: Vec<Vec<f64>> = instances.choose_multiple(&mut rng, k).cloned().collect();
    let mut assignments: Vec<usize> = vec![0; instances.len()];

    for _ in 0..max_iterations {
        // Step 2: Assign clusters
        for (i, instance) in instances.iter().enumerate() {
            let mut min_distance = f64::MAX;
            let mut closest_centroid = 0;
            for (j, centroid) in centroids.iter().enumerate() {
                let distance = instance.iter().zip(centroid.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt();
                if distance < min_distance {
                    min_distance = distance;
                    closest_centroid = j;
                }
            }
            assignments[i] = closest_centroid;
        }

        // Step 3: Update centroids
        let mut new_centroids = vec![vec![0.0; instances[0].len()]; k];
        let mut counts = vec![0; k];
        for (i, instance) in instances.iter().enumerate() {
            let cluster = assignments[i];
            for (j, value) in instance.iter().enumerate() {
                new_centroids[cluster][j] += value;
            }
            counts[cluster] += 1;
        }
        for (i, centroid) in new_centroids.iter_mut().enumerate() {
            for value in centroid.iter_mut() {
                *value /= counts[i] as f64;
            }
        }

        // Check for convergence
        if centroids == new_centroids {
            break;
        }
        centroids = new_centroids;
    }

    assignments
}

pub struct STREAM {}

impl STREAM {
    pub fn new() -> STREAM {
        STREAM {}
    }

    pub fn insert(&mut self, data: Vec<f64>) {
        print!("STREAM: {:?}", data);
    }
}