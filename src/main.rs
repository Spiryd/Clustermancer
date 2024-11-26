mod algorithms;
mod benchmark;
mod samplers;
use benchmark::{processing_rate_benchmark, dimentionality_processing_time_benchmark, benchmark_algorithms_with_samplers, demo_algorithms};

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let options = [
        "Benchmark algorithms",
        "Benchmark algorithms with different dimentionality",
        "Benchmark algorithms with samplers",
        "Demo algorithms",
        "Exit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .default(4)
        .interact()
        .unwrap();
    match selection {
        0 => processing_rate_benchmark(),
        1 => dimentionality_processing_time_benchmark(),
        2 => benchmark_algorithms_with_samplers(),
        3 => demo_algorithms(),
        _ => (),
    }
}
