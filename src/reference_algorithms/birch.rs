use tokio_stream::StreamExt;
use futures::stream::Stream;


#[derive(Debug, Clone)]
struct ClusteringFeature {
    n: usize,
    ls: Vec<f64>,
    ss: Vec<f64>,
}

impl ClusteringFeature {
    fn centroid(self) -> Vec<f64> {
        self.ls.iter().map(|&l| l / self.n as f64).collect()
    }

    fn radius(self) -> Vec<f64> {
        let a: Vec<f64> = self.ss.iter().map(|&s| s / self.n as f64).collect();
        let b: Vec<f64> = self.centroid().iter().map(|&c| c * c).collect();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).sqrt()).collect()
    }

    fn distance_0(self, other: Self) -> Vec<f64> {
        let a: Vec<f64> = self.centroid();
        let b: Vec<f64> = other.centroid();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).abs()).collect()
    }

    fn distance_1(self, other: Self) -> Vec<f64> {
        todo!()
    }

    fn distance_2(self, other: Self) -> Vec<f64> {
        todo!()
    }
}

impl std::ops::Add for ClusteringFeature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let n = self.n + other.n;
        let ls = self.ls.iter().zip(other.ls.iter()).map(|(&a, &b)| a + b).collect();
        let ss = self.ss.iter().zip(other.ss.iter()).map(|(&a, &b)| a + b).collect();
        ClusteringFeature { n, ls, ss }
    }
}

struct CFNode {
    feature: ClusteringFeature,
    children: Vec<CFNode>,
    is_leaf: bool,
}

pub async fn birch<S>(stream: S) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    // Collect results after doubling each item in the stream
    let results: Vec<i32> = stream
        .map(|data| data * 2)
        .collect()
        .await;

    results
}
