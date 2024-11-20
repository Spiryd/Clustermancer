// lambda > 0
const LAMBDA: f64 = 0.2;
const MI: f64 = 0.5;
const EPSILON: f64 = 0.5;
const BETA: f64 = 0.5;

type Point = Vec<f64>;

/// Decay function  f(t) = 2^(−λ·t)
fn decay_function(t: f64) -> f64 {
    2_f64.powf(- LAMBDA * t)
}

/// Euclidean distance between two points
fn distance(a: &Point, b: &Point) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

struct CoreMicroCluster {
    weight: f64,
    center: Point,
    radius: f64,
}

struct PotentialMicroCluster {
    weight: f64,
    cf1: Point,
    cf2: f64,
}

impl PotentialMicroCluster {
    fn new(point: Point) -> PotentialMicroCluster {
        PotentialMicroCluster {
            weight: 1_f64,
            cf1: point.clone(),
            cf2: point,
        }
    }

    fn center(&self) -> Point {
        self.cf1.iter().map(|p_x| p_x / self.weight).collect()
    }

    fn radius(&self) -> f64 {
        f64::sqrt(self)
    }
}

struct OutlierMicroCluster {
    cf1: Point,
    cf2: Point,
    weight: f64,
    t_0: f64,
}

pub struct Denstream {}

impl Denstream {
    pub fn new() -> Denstream {
        Denstream {}
    }

    pub fn insert(&mut self, data: Point) {}
}

impl super::DataStreamClusteringAlgorithm for Denstream {
    fn insert(&mut self, data: Point) {
        self.insert(data);
    }

    fn name(&self) -> String {
        "Denstream".to_string()
    }
}
