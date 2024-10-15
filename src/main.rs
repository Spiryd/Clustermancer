mod reference_algorithms;
use reference_algorithms::birch::Birch;

fn main() {
    let data_stream = (0..100).map(f64::from).map(|x| vec![x]);

    let mut birch = Birch::new(0.5, 2);
    for data in data_stream {
        birch.insert(data);
    }
    println!("{:?}", birch);
}
