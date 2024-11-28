// lambda > 0
const LAMBDA: f64 = 0.2;
const MI: f64 = 2.0;
const EPSILON: f64 = 2.5;
const BETA: f64 = 0.7;
const INIT_N: usize = 100;
const V: usize = 100;
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

fn region_query(data: &[Point], point_idx: usize, eps: f64) -> Vec<usize> {
    data.iter()
        .enumerate()
        .filter(|(idx, point)| distance(&data[point_idx], point) <= eps && *idx != point_idx)
        .map(|(idx, _)| idx)
        .collect()
}

fn initialize_p_micro_clusters(data: &[Vec<f64>], eps: f64, beta_mu: usize) -> Vec<Vec<usize>> {
    let mut visited = vec![false; data.len()];
    let mut clusters = Vec::new();

    for point_idx in 0..data.len() {
        if visited[point_idx] {
            continue;
        }

        let neighbors = region_query(data, point_idx, eps);

        // Only form a micro-cluster if the density is above beta_mu
        if neighbors.len() + 1 >= beta_mu {
            let mut micro_cluster = Vec::new();
            visited[point_idx] = true;

            for &neighbor_idx in &neighbors {
                if !visited[neighbor_idx] {
                    micro_cluster.push(neighbor_idx);
                    visited[neighbor_idx] = true;
                }
            }

            clusters.push(micro_cluster);
        }
    }

    clusters
}

#[derive(Debug, Clone)]
pub struct PotentialMicroCluster {
    weight: f64,
    cf1: Point,
    cf2: f64,
    last_update: usize,
}

impl PotentialMicroCluster {
    fn new(points: Vec<Point>) -> PotentialMicroCluster {
        let weight = points.len() as f64;
        let cf1 = points.iter().fold(vec![0_f64; points[0].len()], |acc, p| {
            acc.iter().zip(p.iter()).map(|(x, y)| x + y).collect()
        });
        let cf2 = points
            .iter()
            .map(|p| p.iter().map(|x| x.powi(2)).sum::<f64>())
            .sum::<f64>();
        let last_update = 0;
        PotentialMicroCluster {
            weight,
            cf1,
            cf2,
            last_update,
        }
    }

    fn from_outlier(outlier: OutlierMicroCluster) -> PotentialMicroCluster {
        PotentialMicroCluster {
            weight: outlier.weight,
            cf1: outlier.cf1,
            cf2: outlier.cf2,
            last_update: outlier.last_update,
        }
    }

    pub fn center(&self) -> Point {
        self.cf1.iter().map(|p_x| p_x / self.weight).collect()
    }

    pub fn radius(&self) -> f64 {
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
    initial_buffer: Vec<Point>,
    initialised: bool,
    potential_micro_clusters: Vec<PotentialMicroCluster>,
    outlier_micro_clusters: Vec<OutlierMicroCluster>,
    t_p: usize,
    clock: usize,
    small_clock: usize,
}

impl Denstream {
    pub fn new() -> Denstream {
        Denstream {
            initial_buffer: Vec::new(),
            initialised: false,
            potential_micro_clusters: Vec::new(),
            outlier_micro_clusters: Vec::new(),
            t_p: calculate_t_p(),
            clock: 0,
            small_clock: 0,
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
                        .push(PotentialMicroCluster::from_outlier(after_merge_outlier));
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
        if !self.initialised {
            self.initial_buffer.push(data);
            if self.initial_buffer.len() >= INIT_N {
                self.initialised = true;
                let mappings = initialize_p_micro_clusters(
                    &self.initial_buffer,
                    EPSILON,
                    (BETA * MI) as usize,
                );
                for mapping in mappings {
                    if !mapping.is_empty() {
                        self.potential_micro_clusters
                            .push(PotentialMicroCluster::new(
                                mapping
                                    .iter()
                                    .map(|&idx| self.initial_buffer[idx].clone())
                                    .collect(),
                            ));
                    }
                }
                self.initial_buffer.clear();
            }
            return;
        }
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
        if self.small_clock % V == 0 {
            self.clock += 1;
        }
        self.small_clock += 1;
    }

    fn is_directly_density_reachable(
        &self,
        cp: &PotentialMicroCluster,
        cq: &PotentialMicroCluster,
        epsilon: f64,
        mu: f64,
    ) -> bool {
        cq.weight >= mu && distance(&cp.center(), &cq.center()) <= 2.0 * epsilon
    }

    fn is_density_reachable(
        &self,
        cp: &PotentialMicroCluster,
        cq: &PotentialMicroCluster,
        epsilon: f64,
        mu: f64,
    ) -> bool {
        let mut visited = Vec::new();
        let mut to_visit = vec![cq];

        while let Some(current) = to_visit.pop() {
            if visited.iter().any(|v| v == &current.center()) {
                continue;
            }
            visited.push(current.center().clone());

            if self.is_directly_density_reachable(cp, current, epsilon, mu) {
                return true;
            }

            for neighbor in &self.potential_micro_clusters {
                if self.is_directly_density_reachable(current, neighbor, epsilon, mu) {
                    to_visit.push(neighbor);
                }
            }
        }

        false
    }

    pub fn clustering_request(&self) -> Vec<Vec<PotentialMicroCluster>> {
        let mut clusters = Vec::new();
        let mut visited = Vec::new();

        for cp in &self.potential_micro_clusters {
            if visited.iter().any(|v| v == &cp.center()) {
                continue;
            }

            let mut cluster = Vec::new();
            let mut to_visit = vec![cp];

            while let Some(current) = to_visit.pop() {
                if visited.iter().any(|v| v == &current.center()) {
                    continue;
                }
                visited.push(current.center().clone());
                cluster.push(current.clone());

                for neighbor in &self.potential_micro_clusters {
                    if self.is_density_reachable(current, neighbor, EPSILON * 2.7, MI) {
                        to_visit.push(neighbor);
                    }
                }
            }

            clusters.push(cluster);
        }

        clusters
    }
}

impl super::DataStreamClusteringAlgorithm for Denstream {
    fn insert(&mut self, data: Point) {
        self.insert(data);
    }
    fn clusters(&self) -> Vec<super::ClusteringElement> {
        let clusters = self.clustering_request();
        clusters
            .iter()
            .enumerate()
            .flat_map(|(idx, cluster)| {
                cluster.iter().map(move |c| super::ClusteringElement {
                    center: c.center(),
                    radius: c.radius(),
                    cluster: idx,
                })
            })
            .collect()
    }
    fn name(&self) -> String {
        "DenStream".to_string()
    }
}
