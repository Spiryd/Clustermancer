mod algorithms;
mod benchmark;
mod samplers;
use benchmark::*;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let options = [
        "Benchmark algorithms",
        "Benchmark algorithms with different dimentionality",
        "Benchmark quality(real data)",
        "Benchmark quality(synthetic data)",
        "Benchmark algorithms with samplers",
        "Demo algorithms",
        "Demo samplers",
        "Exit",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    match selection {
        0 => processing_rate_benchmark(),
        1 => dimentionality_processing_time_benchmark(),
        2 => real_quality_benchmark(),
        3 => synthetic_quality_benchmark(),
        4 => processing_rate_samplers_benchmark(),
        5 => demo_algorithms(),
        6 => demo_samplers(),
        _ => (),
    }
}
