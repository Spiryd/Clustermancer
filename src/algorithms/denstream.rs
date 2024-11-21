// lambda > 0
const LAMBDA: f64 = 0.1;
const MI: f64 = 2.0;
const EPSILON: f64 = 2.;
const BETA: f64 = 0.6;
type Point = Vec<f64>;

fn calculate_t_p() -> usize {
    ((1_f64 / LAMBDA) * ((BETA * MI) / ((BETA * MI) - 1_f64)).log2()).ceil() as usize
}

/// Decay function  f(t) = 2^(−λ·t)
fn decay_function(t: f64) -> f64 {
    2_f64.powf(-LAMBDA * t)
}

/// Euclidean distance between two points
fn distance(a: &Point, b: &Point) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[derive(Debug, Clone)]
struct CoreMicroCluster {
    weight: f64,
    center: Point,
    radius: f64,
}

#[derive(Debug, Clone)]
struct PotentialMicroCluster {
    weight: f64,
    cf1: Point,
    cf2: f64,
    last_update: usize,
}

impl PotentialMicroCluster {
    fn new(outlier: OutlierMicroCluster) -> PotentialMicroCluster {
        PotentialMicroCluster {
            weight: outlier.weight,
            cf1: outlier.cf1,
            cf2: outlier.cf2,
            last_update: outlier.last_update,
        }
    }

    fn center(&self) -> Point {
        self.cf1.iter().map(|p_x| p_x / self.weight).collect()
    }

    fn radius(&self) -> f64 {
        f64::sqrt(self.cf2 / self.weight - self.center().iter().map(|x| x.powi(2)).sum::<f64>())
    }

    fn update(&mut self, timestamp: usize) {
        let decay = decay_function((timestamp - self.last_update) as f64);
        self.weight = self.weight * decay;
        self.cf1 = self.cf1.iter().map(|x| x * decay).collect();
        self.cf2 *= decay;
        self.last_update = timestamp;
    }

    fn add_point(&mut self, point: Point) {
        self.weight += 1_f64;
        self.cf1 = self
            .cf1
            .iter()
            .zip(point.iter())
            .map(|(x, y)| x + y)
            .collect();
        self.cf2 += point.iter().map(|x| x.powi(2)).sum::<f64>();
    }

    fn get_after_merge(&self, point: Point) -> Self {
        let mut potential = self.clone();
        potential.add_point(point);
        potential
    }
}

#[derive(Debug, Clone)]
struct OutlierMicroCluster {
    cf1: Point,
    cf2: f64,
    weight: f64,
    t_0: usize,
    last_update: usize,
}

impl OutlierMicroCluster {
    fn new(point: Point, timestamp: usize) -> OutlierMicroCluster {
        OutlierMicroCluster {
            cf1: point.clone(),
            cf2: point.iter().map(|x| x.powi(2)).sum::<f64>(),
            weight: 1_f64,
            t_0: timestamp,
            last_update: timestamp,
        }
    }

    fn center(&self) -> Point {
        self.cf1.iter().map(|p_x| p_x / self.weight).collect()
    }

    fn radius(&self) -> f64 {
        f64::sqrt(self.cf2 / self.weight - self.center().iter().map(|x| x.powi(2)).sum::<f64>())
    }

    fn update(&mut self, timestamp: usize) {
        let decay = decay_function((timestamp - self.last_update) as f64);
        self.weight *= decay;
        self.cf1 = self.cf1.iter().map(|x| x * decay).collect();
        self.cf2 *= decay;
        self.last_update = timestamp;
    }

    fn add_point(&mut self, point: Point) {
        self.weight += 1_f64;
        self.cf1 = self
            .cf1
            .iter()
            .zip(point.iter())
            .map(|(x, y)| x + y)
            .collect();
        self.cf2 += point.iter().map(|x| x.powi(2)).sum::<f64>();
    }

    fn get_after_merge(&self, point: Point) -> Self {
        let mut potential = self.clone();
        potential.add_point(point);
        potential
    }
}

#[derive(Debug)]
pub struct Denstream {
    potential_micro_clusters: Vec<PotentialMicroCluster>,
    outlier_micro_clusters: Vec<OutlierMicroCluster>,
    t_p: usize,
    clock: usize,
}

impl Denstream {
    pub fn new() -> Denstream {
        Denstream {
            potential_micro_clusters: Vec::new(),
            outlier_micro_clusters: Vec::new(),
            t_p: calculate_t_p(),
            clock: 0,
        }
    }

    fn merge(&mut self, data: Point) {
        // Try to merge with potential micro-clusters
        self.potential_micro_clusters
            .iter_mut()
            .for_each(|pcm| pcm.update(self.clock));
        if let Some((idx, _)) = self
            .potential_micro_clusters
            .iter()
            .enumerate()
            .map(|(i, cluster)| (i, distance(&cluster.center(), &data)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            let after_merge_potential =
                self.potential_micro_clusters[idx].get_after_merge(data.clone());
            if after_merge_potential.radius() <= EPSILON {
                self.potential_micro_clusters[idx] = after_merge_potential;
                return;
            }
        }
        // else: try to merge with outlier micro-clusters
        self.outlier_micro_clusters
            .iter_mut()
            .for_each(|omc| omc.update(self.clock));

        if let Some((idx, _)) = self
            .outlier_micro_clusters
            .iter()
            .enumerate()
            .map(|(i, cluster)| (i, distance(&cluster.center(), &data)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            let after_merge_outlier =
                self.outlier_micro_clusters[idx].get_after_merge(data.clone());
            if after_merge_outlier.radius() <= EPSILON {
                if after_merge_outlier.weight > BETA * MI {
                    self.outlier_micro_clusters.remove(idx);
                    self.potential_micro_clusters
                        .push(PotentialMicroCluster::new(after_merge_outlier));
                } else {
                    self.outlier_micro_clusters[idx] = after_merge_outlier;
                }
                return;
            }
        }
        // else: create new outlier micro-cluster
        self.outlier_micro_clusters
            .push(OutlierMicroCluster::new(data, self.clock));
    }

    pub fn insert(&mut self, data: Point) {
        // 1. Merge data point with potential micro-clusters
        self.merge(data);
        if self.clock % self.t_p == 0 {
            // Prune old potential micro-clusters
            self.potential_micro_clusters
                .retain(|cmp| cmp.weight >= BETA * MI);
            // Prune old outlier micro-clusters
            let mut indexes_to_remove: Vec<usize> = Vec::new();
            for (idx, omp) in self.outlier_micro_clusters.iter().enumerate() {
                let xi = (2_f64
                    .powf(-LAMBDA * (self.clock as f64 - omp.t_0 as f64 + self.t_p as f64))
                    - 1.)
                    / (2_f64.powf(-LAMBDA * self.t_p as f64) - 1.);
                if omp.weight * xi < BETA * MI {
                    indexes_to_remove.push(idx);
                }
            }
            indexes_to_remove.sort_unstable_by(|a, b| b.cmp(a));
            indexes_to_remove.iter().for_each(|idx| {
                self.outlier_micro_clusters.remove(*idx);
            });

        }
        self.clock += 1;
    }

    pub fn clustering_request(&mut self) {}

    pub fn print_state(&self) {
        println!("Potential micro-cluster count: {}", self.potential_micro_clusters.len());
        println!("Outlier micro-cluster count: {}", self.outlier_micro_clusters.len());
    }
}

impl super::DataStreamClusteringAlgorithm for Denstream {
    fn insert(&mut self, data: Point) {
        self.insert(data);
    }

    fn name(&self) -> String {
        "Denstream".to_string()
    }
}
