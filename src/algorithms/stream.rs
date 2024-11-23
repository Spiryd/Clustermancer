use rand::prelude::*;
use rand_pcg::Pcg64;

type Point = Vec<f64>;

const M: usize = 200;
const ALPHA: usize = 2;

const EPSILON: f64 = 0.1;
const EPSILON_PRIME: f64 = 0.1;
const EPSILON_DOUBLE_PRIME: f64 = 0.1;

fn distance(a: &Point, b: &Point) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

pub struct Stream {
    k: usize,
    buffer: Vec<Point>,
    _intermediate_median_points: Vec<(Point, usize)>,
    rng: Pcg64,
}

impl Stream {
    pub fn new(k: usize) -> Stream {
        Stream {
            k,
            buffer: Vec::new(),
            _intermediate_median_points: Vec::new(),
            rng: Pcg64::from_entropy(),
        }
    }

    fn initial_solution(
        &mut self,
        data_set: &[Point],
        facility_cost: f64,
    ) -> (Vec<usize>, Vec<usize>) {
        let mut shuffled_indexes: Vec<usize> = (0..data_set.len()).collect();
        shuffled_indexes.shuffle(&mut self.rng);
        let mut clusters: Vec<usize> = Vec::new();
        let mut assignmesnts: Vec<usize> = vec![0; data_set.len()];
        clusters.push(shuffled_indexes.pop().unwrap());
        assignmesnts[clusters[0]] = clusters[0];
        while let Some(idx_x) = shuffled_indexes.pop() {
            let (closest_idx, closest_distance) = clusters
                .iter()
                .map(|&idx_y| (idx_y, distance(&data_set[idx_x], &data_set[idx_y])))
                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                .unwrap();
            if self.rng.gen_bool(closest_distance / facility_cost) {
                clusters.push(idx_x);
                assignmesnts[idx_x] = idx_x;
            } else {
                assignmesnts[idx_x] = closest_idx;
            }
        }
        (clusters, assignmesnts)
    }

    fn facility_location(
        &mut self,
        data_set: &Vec<Point>,
        facility_cost: f64,
        epsilon: f64,
        (i, a): (Vec<usize>, Vec<usize>),
    ) -> (Vec<usize>, Vec<usize>) {
        let mut cost = facility_cost * (i.len() as f64)
            + data_set
                .iter()
                .enumerate()
                .map(|(i, x)| distance(x, &data_set[a[i]]))
                .sum::<f64>();
        let mut shuffled_indexes: Vec<usize> = (0..data_set.len()).collect();

        let mut potential_i = i.clone();
        let mut potential_a = a.clone();

        loop {
            shuffled_indexes.shuffle(&mut self.rng);

            for &idx_y in &shuffled_indexes {
                // Calcualate gain
                let mut gain = 0.0;
                let mut potential_potential_i = potential_i.clone();
                let mut potential_potential_a = potential_a.clone();
                if i.contains(&idx_y) {
                    // Gain for removing y
                    gain += facility_cost;
                    potential_potential_i.retain(|&x| x != idx_y);
                    let children = potential_potential_a
                        .iter()
                        .enumerate()
                        .filter(|(_, &x)| x == idx_y)
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
                    for child in children {
                        let (facility_mapping, mapping_cost) = potential_potential_i
                            .iter()
                            .map(|&facility| {
                                (facility, distance(&data_set[child], &data_set[facility]))
                            })
                            .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                            .unwrap();
                        gain += mapping_cost;
                        potential_potential_a[child] = facility_mapping;
                    }
                    let (facility_mapping, mapping_cost) = potential_potential_i
                        .iter()
                        .map(|&facility| {
                            (facility, distance(&data_set[idx_y], &data_set[facility]))
                        })
                        .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                        .unwrap();
                    gain -= mapping_cost;
                    potential_potential_a[idx_y] = facility_mapping;
                } else {
                    // Gain for adding y
                    gain -= facility_cost;
                    potential_potential_i.push(idx_y);
                    for idx_x in 0..data_set.len() {
                        let distance_x_y = distance(&data_set[idx_x], &data_set[idx_y]);
                        if distance_x_y
                            < distance(&data_set[idx_x], &data_set[potential_potential_a[idx_x]])
                        {
                            potential_potential_a[idx_x] = idx_y;
                            gain += distance_x_y
                                - distance(
                                    &data_set[idx_x],
                                    &data_set[potential_potential_a[idx_x]],
                                );
                        }
                    }
                    // check if any facility is redundant
                    let mut counts = vec![0; data_set.len()];
                    for &idx in &potential_potential_a {
                        counts[idx] += 1;
                    }
                    for (idx, &count) in counts.iter().enumerate() {
                        if count == 1 {
                            // check if close idx is beneficial
                            let (facility_mapping, mapping_cost) = potential_potential_i
                                .iter()
                                .map(|&facility| {
                                    (facility, distance(&data_set[idx], &data_set[facility]))
                                })
                                .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())
                                .unwrap();
                            if mapping_cost < facility_cost {
                                gain += facility_cost;
                                gain -= mapping_cost;
                                potential_potential_a[idx] = facility_mapping;
                                potential_potential_i.retain(|&x| x != idx);
                            }
                        }
                    }
                }
                if gain > 0.0 {
                    potential_a = potential_potential_a;
                    potential_i = potential_potential_i;
                    println!(
                        "new cost: {:?}",
                        facility_cost * (potential_i.len() as f64)
                            + data_set
                                .iter()
                                .enumerate()
                                .map(|(i, x)| distance(x, &data_set[potential_a[i]]))
                                .sum::<f64>()
                    )
                }
            }
            let new_cost = facility_cost * (potential_i.len() as f64)
                + data_set
                    .iter()
                    .enumerate()
                    .map(|(i, x)| distance(x, &data_set[potential_a[i]]))
                    .sum::<f64>();
            if new_cost <= cost * (1.0 - epsilon) {
                cost = new_cost;
            } else {
                break;
            }
        }
        (potential_i, potential_a)
    }

    fn lsearch(&mut self, data_set: &Vec<Point>) -> (Vec<Point>, Vec<usize>) {
        // x_0 an arbitrary point in N
        let x_0 = data_set.choose(&mut self.rng).unwrap();
        // 1. z_min = 0
        let mut z_min = 0.0;
        // 2. z_max = sum(distance(x, x_0) for x in data_set)
        let mut z_max: f64 = data_set.iter().map(|x| distance(x, x_0)).sum::<f64>();
        // 3. z = (z_min + z_max) / 2
        let mut z = (z_min + z_max) / 2.0;
        // 4. InitialSolution
        let (mut i, a) = self.initial_solution(data_set, z);
        // 5. Randomly pick Θ( 1/p log k) points as feasible medians
        // let mut feasible_medians: Vec<usize> = i.choose_multiple(&mut self.rng, (self.k as f64).log2() as usize).cloned().collect();
        let mut feasible_medians: Vec<usize> = Vec::new();
        while feasible_medians.len() < (self.k as f64).log2().ceil() as usize {
            let idx = self.rng.gen_range(0..data_set.len());
            if !feasible_medians.contains(&idx) {
                feasible_medians.push(idx);
            }
        }
        for idx in feasible_medians {
            if !i.contains(&idx) {
                i.push(idx);
            }
        }
        // 6. While #medians =/= k and z_min < (1 - eplsion'')z_max
        let (mut f, mut g) = (i, a);

        while f.len() != self.k && z_min < (1.0 - EPSILON_DOUBLE_PRIME) * z_max {
            let (mut f_prim, mut g_prim) =
                self.facility_location(data_set, z, EPSILON, (f.clone(), g.clone()));
            if f_prim.len() == self.k {
                (f_prim, g_prim) =
                    self.facility_location(data_set, z, EPSILON_PRIME, (f_prim, g_prim));
            }
            if f_prim.len() > self.k {
                z_min = z;
                z = (z_min + z_max) / 2.0;
            } else if f_prim.len() < self.k {
                z_max = z;
                z = (z_min + z_max) / 2.0;
            }
            f = f_prim;
            g = g_prim;
        }
        let mut centroids: Vec<Point> = Vec::new();
        for facility in f {
            // compute centeroid
            let cluster: Vec<Point> = data_set
                .iter()
                .enumerate()
                .filter(|x| g[x.0] == facility)
                .map(|x| x.1)
                .cloned()
                .collect();
            if cluster.is_empty() {
                continue;
            }
            centroids.push(
                cluster
                    .iter()
                    .cloned()
                    .reduce(|a, b| {
                        a.iter()
                            .zip(b.iter())
                            .map(|(x, y)| x + y)
                            .collect::<Point>()
                    })
                    .unwrap()
                    .iter()
                    .map(|x| x / (cluster.len() as f64))
                    .collect(),
            );
        }

        (centroids, g)
    }

    pub fn insert(&mut self, data: Point) {
        self.buffer.push(data);
        if self.buffer.len() >= (M / self.k) * ALPHA {
            println!("{:?}", self.lsearch(&self.buffer.clone()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        assert_eq!(distance(&a, &b), 5.0);
    }

    #[test]
    fn test_initial_solution() {
        let data_set = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
            vec![5.0, 5.0],
            vec![6.0, 6.0],
            vec![1.0, 0.0],
            vec![0.0, 1.0],
            vec![4.0, 5.0],
        ];
        let x_0 = data_set.choose(&mut thread_rng()).unwrap();
        let z_max: f64 = data_set.iter().map(|x| distance(x, &x_0)).sum::<f64>();
        let z = (0.0 + z_max) / 2.0;
        let mut stream = Stream::new(2);
        let (clusters, assignments) = stream.initial_solution(&data_set, z);
        println!("{:?}", clusters);
        println!("{:?}", assignments);
    }
}
