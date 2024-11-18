use rand::prelude::*;
use rand_pcg::Pcg64;

type Point = Vec<f64>;

const M: usize = 100;
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
    intermediate_median_points: Vec<Point>,
    rng: Pcg64,
}

impl Stream {
    pub fn new(k: usize) -> Stream {
        Stream {
            k,
            buffer: Vec::new(),
            intermediate_median_points: Vec::new(),
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
        let total_cost = facility_cost * (i.len() as f64)
            + data_set
                .iter()
                .enumerate()
                .map(|(i, x)| distance(x, &data_set[a[i]]))
                .sum::<f64>();
        let mut improvement = true;
        let mut feasables: Vec<usize> = (0..data_set.len()).collect();
        while improvement {
            improvement = false;
            feasables.shuffle(&mut self.rng);
            for &y_idx in &feasables {
                let gain = if i.contains(&y_idx) { 
                    facility_cost - distance(&data_set[y_idx], &data_set[a[y_idx]])
                } else { 
                    distance(&data_set[y_idx], &data_set[a[y_idx]]) - facility_cost
                };
            }
        }
        todo!()
    }

    fn lsearch(&mut self, data_set: &Vec<Point>, k: usize) {
        // x_0 an arbitrary point in N
        let x_0 = data_set.choose(&mut self.rng).unwrap();
        // 1. z_min = 0
        let z_min = 0.0;
        // 2. z_max = sum(distance(x, x_0) for x in data_set)
        let z_max: f64 = data_set.iter().map(|x| distance(x, x_0)).sum::<f64>();
        // 3. z = (z_min + z_max) / 2
        let z = (z_min + z_max) / 2.0;
        // 4. InitialSolution
        let (i, a) = self.initial_solution(data_set, z);
        // 5. Randomly pick Θ( 1/p log k) points as feasible medians
        // let mut feasible_medians: Vec<usize> = i.choose_multiple(&mut self.rng, (self.k as f64).log2() as usize).cloned().collect();
        // 6. While #medians =/= k and z_min < (1 - eplsion'')z_max
        let (f, g) = (i, a);
        while f.len() != self.k && z_min < (1.0 - EPSILON_DOUBLE_PRIME) * z_max {
            let (f_prim, g_prim) =
                self.facility_location(data_set, z, EPSILON, (f.clone(), g.clone()));
        }
    }

    pub fn insert(&mut self, data: Point) {
        self.buffer.push(data);
        if self.buffer.len() >= (M / self.k) * ALPHA {
            // todo: implement the algorithm
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
