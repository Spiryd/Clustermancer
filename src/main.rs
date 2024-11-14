use rand::prelude::*;

mod algorithms;
use algorithms::birch::Birch;
use algorithms::clustream::CluStream;
use algorithms::stream::STREAM;

fn main() {
    let birch = Birch::new(5., 2);
    let clustream = CluStream::new();
    let mut stream = STREAM::new();

    for (idx, data) in (0..10_000_000).map(|_| thread_rng().gen_range(0..100)).map(|x| vec![x as f64]).enumerate() {
        //clustream.insert(data.clone());
        if idx % 10_000 == 0 {
            println!("Item: {:#?}", idx);
        }
        //birch.insert(data.clone());
        stream.insert(data);
    }
    birch.print_tree();
    clustream.pirnt_centroids();
}
