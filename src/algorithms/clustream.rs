use rand::prelude::*;
use rand_pcg::Pcg64;
use statrs::distribution::{Normal, ContinuousCDF};
use itertools::Itertools;

/// Memory size
const Q: usize = 10;
/// Max kmeans iterations
const MAX_ITERATIONS: usize = 10_000;
/// Number of points to initialize the micro-clusters
const INIT_NUMBER: usize = 10;
/// Maximum boundary factor
const MAXIMUM_BOUNDARY_FACTOR: f64 = 2.;
/// Threshold for the merge operation
const THRESHOLD: f64 = 0.5;
/// How far do we look back in the stream for outliers
const M: usize = 10;
// storage params
const ALPHA: usize = 2;
const L: usize = 2;

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

    fn centroid(&self) -> Vec<f64> {
        self.cf1x.iter().map(|x| x / self.n as f64).collect()
    }

    fn distance(&self, instance: &Vec<f64>) -> f64 {
        let centroid = self.centroid();
        instance.iter().zip(centroid.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
    }

    fn maximal_boundary(&self) -> Option<f64> {
        if self.n > 1 {
            let a = self.cf2x.iter().map(|x_p| x_p / self.n as f64).collect::<Vec<f64>>();
            let b = self.centroid().iter().map(|x_p| x_p.powi(2)).collect::<Vec<f64>>();
            return Some(MAXIMUM_BOUNDARY_FACTOR * (a.iter().zip(b.iter()).map(|(x, y)| (x - y) / a.len() as f64).sum::<f64>().sqrt()));
        }
        None
    }

    fn relevance_stamp(&self) -> f64 {
        let mean = self.cf1t / self.n as f64;
        if self.n < 2 * M {
            return mean;
        }
        let standard_deviation = (self.cf2t / self.n as f64 - mean.powi(2)).sqrt();
        let procentile = M as f64 / (2 * self.n) as f64;
        let normal = Normal::new(mean, standard_deviation).unwrap();
        normal.inverse_cdf(procentile)
    }
}

impl std::ops::Add for MicroCluster {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        MicroCluster {
            cf2x: self.cf2x.iter().zip(other.cf2x.iter()).map(|(x, y)| x + y).collect(),
            cf1x: self.cf1x.iter().zip(other.cf1x.iter()).map(|(x, y)| x + y).collect(),
            cf2t: self.cf2t + other.cf2t,
            cf1t: self.cf1t + other.cf1t,
            n: self.n + other.n,
        }
    }
}

impl std::ops::AddAssign for MicroCluster {
    fn add_assign(&mut self, other: Self) {
        self.cf2x = self.cf2x.iter().zip(other.cf2x.iter()).map(|(x, y)| x + y).collect();
        self.cf1x = self.cf1x.iter().zip(other.cf1x.iter()).map(|(x, y)| x + y).collect();
        self.cf2t += other.cf2t;
        self.cf1t += other.cf1t;
        self.n += other.n;
    }
}

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

#[derive(Debug, Clone)]
struct Snapshot {
    timestamp: usize,
    micro_clusters: Vec<(MicroCluster, Vec<usize>)>,
}

#[derive(Debug)]
struct SnapshotVault {
    snapshots: Vec<(Vec<Option<Snapshot>>, usize)>,
}

impl SnapshotVault {
    fn new() -> Self {
        SnapshotVault {
            snapshots: Vec::new(),
        }
    }

    fn insert(&mut self, snapshot: Snapshot) {
        let order = if ALPHA == 2 {
            snapshot.timestamp.trailing_zeros() as usize
        } else {
            Self::find_orders(snapshot.timestamp)
        };
        if let Some((snapshots, idx)) = self.snapshots.get_mut(order) {
            snapshots[*idx] = Some(snapshot);
            *idx = (*idx + 1) % 5;
        } else {
            println!("New order {}", order);
            let mut snapshots = vec![None; 5];
            snapshots[0] = Some(snapshot);
            self.snapshots.insert(order, (snapshots, 1));
        }
    }

    fn find_orders(clock_time: usize) -> usize {
        let mut i: usize = 0;
        let mut tmp_clock_time = clock_time;
        while tmp_clock_time % ALPHA == 0 {
            tmp_clock_time /= ALPHA;
            i += 1;
        }
        i
    }
    
}

#[derive(Debug)]
pub struct CluStream{
    snapshot_vault: SnapshotVault,
    micro_clusters: Vec<(MicroCluster, Vec<usize>)>,
    initiated: bool,
    initial_buffer: Vec<Vec<f64>>,
    clock: usize,
    next_id: usize,
}

impl CluStream {
    pub fn new() -> Self {
        CluStream {
            snapshot_vault: SnapshotVault::new(),
            micro_clusters: Vec::new(),
            initiated: false,
            initial_buffer: Vec::new(),
            clock: 1,
            next_id: 0,
        }
    }

    pub fn insert(&mut self, instance: Vec<f64>) {
        if !self.initiated {
            // Step 1: Initialize micro-clusters with the first INIT_NUMBER points using k-means
            self.initial_buffer.push(instance);
            if self.initial_buffer.len() == INIT_NUMBER {
                self.initiated = true;
                let initial_micro_cluster_mapping = kmeans(self.initial_buffer.clone(), Q, MAX_ITERATIONS);
                let mut micro_clusters: Vec<Option<MicroCluster>> = vec![None; Q];
                for (i, group) in initial_micro_cluster_mapping.iter().enumerate() {
                    if let Some(micro_cluster) = &mut micro_clusters[*group] {
                        *micro_cluster += MicroCluster::new(self.initial_buffer[i].clone(), i);
                    } else {
                        micro_clusters[*group] = Some(MicroCluster::new(self.initial_buffer[i].clone(), i));
                    }
                }
                self.micro_clusters = micro_clusters.into_iter().flatten().enumerate().map(|(i, mc)| (mc, vec![i])).collect();
                self.next_id = self.micro_clusters.len();
                self.initial_buffer.clear();
            }
        } else {
            // Step 2: Update micro-clusters
            let min_cluster_idx = self.micro_clusters.iter().enumerate().min_by(|(_, a), (_, b)| a.0.distance(&instance).partial_cmp(&b.0.distance(&instance)).unwrap()).unwrap().0;
            let max_boundary = match self.micro_clusters[min_cluster_idx].0.maximal_boundary() {
                Some(boundary) => boundary,
                None => {
                    let centroid = self.micro_clusters[min_cluster_idx].0.centroid();
                    self.micro_clusters.iter().map(|(a, _)|a.distance(&centroid)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
                }
            };
            if self.micro_clusters[min_cluster_idx].0.distance(&instance) <= max_boundary {
                self.micro_clusters[min_cluster_idx].0 += MicroCluster::new(instance, self.clock);
                // println!("Added to cluster {}", self.micro_clusters[min_cluster_idx].1);
            } else if self.micro_clusters.len() < Q {
                println!("New cluster(Quota not reached)");
                self.micro_clusters.push((MicroCluster::new(instance, self.clock), vec![self.next_id]));
                self.next_id += 1;
            } else {
                let least_relevant = self.micro_clusters.iter().enumerate().map(|(i, (mc, _))| (i, mc.relevance_stamp())).min_by(|(_, rel_a), (_, rel_b)| rel_a.partial_cmp(rel_b).unwrap()).unwrap();
                if least_relevant.1 < THRESHOLD{
                    // Prune outliers
                    self.micro_clusters[least_relevant.0] = (MicroCluster::new(instance, self.clock), vec![self.next_id]);
                    println!("Replaced least relevant with id {}", self.next_id);
                    self.next_id += 1;
                } else {
                    // Merge
                    // println!("Merging");
                    let closest_pair = self.micro_clusters.iter().enumerate().combinations(2).map(|x| ((x[0].0, x[1].0), x[0].1.0.distance(&x[1].1.0.centroid()))).min_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap().0;
                    let merge_from = self.micro_clusters[closest_pair.1].clone();
                    self.micro_clusters[closest_pair.0].0 += merge_from.0;
                    self.micro_clusters[closest_pair.0].1.extend(merge_from.1);
                    self.micro_clusters[closest_pair.1].0 = MicroCluster::new(instance, self.clock);
                    self.micro_clusters[closest_pair.1].1 = vec![self.next_id];
                    self.next_id += 1;
                }
            }
        }
        // Step 3: Create snapshots
        self.snapshot_vault.insert(Snapshot { timestamp: self.clock, micro_clusters: self.micro_clusters.clone()});
        // Get ready for the next iteration
        self.clock += 1;
    }

    pub fn offline_macro_clustering(&self, h: usize, k: usize) {
        todo!()
    }

    pub fn pirnt_centroids(&self) {
        for (i, (mc, _)) in self.micro_clusters.iter().enumerate() {
            println!("Centroid {}: {:?}", i, mc.centroid());
        }
    }

    pub fn print_vault(&self) {
        for (i, (snapshots, _)) in self.snapshot_vault.snapshots.iter().enumerate() {
            print!("Order {}:", i);
            for snapshot in snapshots {
                if let Some(snapshot) = snapshot {
                    print!(" {},", snapshot.timestamp);
                }
            }
            println!("");
        }
    }
}

impl super::DataStreamClusteringAlgorithm for CluStream {
    fn insert(&mut self, data: Vec<f64>) {
        self.insert(data);
    }
    fn name(&self) -> String {
        format!("CluStream")
    }
}