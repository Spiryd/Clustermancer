use rand::prelude::*;

mod algorithms;
use algorithms::birch::Birch;
use algorithms::clustream::CluStream;

fn main() {
    let birch = Birch::new(5., 2);
    let mut clustream = CluStream::new();
    for data in (0..100_000).map(|_| thread_rng().gen_range(0..100)).map(|x| vec![x as f64]) {
        clustream.insert(data.clone());
        // birch.insert(data);
    }
    birch.print_tree();
    clustream.pirnt_centroids();
}
