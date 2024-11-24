mod algorithms;
mod benchmark;
mod samplers;
use benchmark::{benchmark_algorithms, benchmark_algorithms_with_samplers, demo_algorithms};

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let options = [
        "Benchmark algorithms",
        "Benchmark algorithms with samplers",
        "Demo algorithms",
        "Exit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => benchmark_algorithms(),
        1 => benchmark_algorithms_with_samplers(),
        2 => demo_algorithms(),
        _ => (),
    }
}
