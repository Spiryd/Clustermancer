mod reference_algorithms;
use reference_algorithms::birch::Birch;
use reference_algorithms::clustream::CluStream;

fn main() {
    let data_stream: Vec<Vec<f64>> = [22,9,12,15,18,27,11,36,10,3,14,32].iter().map(|x| vec![*x as f64]).collect();
    let mut birch = Birch::new(5., 2);
    for data in data_stream {
        birch.insert(data);
        birch.print_tree();
    }
}
