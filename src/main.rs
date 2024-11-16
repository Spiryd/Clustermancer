mod algorithms;
mod benchmark;
mod samplers;
use benchmark::{benchmark_algorithms, benchmark_algorithms_with_samplers};

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let options = [
        "Test",
        "Benchmark algorithms",
        "Benchmark algorithms with samplers",
        "Exit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => test(),
        1 => benchmark_algorithms(),
        2 => benchmark_algorithms_with_samplers(),
        _ => (),
    }
}

fn test() {
    use algorithms::clustream::CluStream;
    use rand::{thread_rng, Rng};

    let mut clustream = CluStream::new();
    let h: usize = 1_000_000;
    for (idx, data) in (0..h)
        .map(|_| thread_rng().gen_range(0..100))
        .map(|x| vec![x as f64])
        .enumerate()
    {
        clustream.insert(data.clone());
        if idx % 10_000 == 0 {
            println!("Item: {:#?}", idx);
        }
    }
    clustream.print_vault();
    clustream.offline_macro_clustering(h, 6);
}
