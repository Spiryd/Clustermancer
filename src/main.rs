mod algorithms;
mod benchmark;
mod samplers;
use benchmark::*;

use dialoguer::{theme::ColorfulTheme, Select};

use std::fs;

fn main() {
    fs::create_dir_all("./benchmark_results").unwrap();
    fs::create_dir_all("./demo_results").unwrap();
    let options = [
        "Benchmark algorithms",
        "Benchmark algorithms with different dimentionality",
        "Benchmark quality(real data)",
        "Benchmark quality(synthetic data)",
        "Benchmark algorithms with samplers",
        "Benchmark samplers quality(real data)",
        "Demo algorithms",
        "Demo samplers",
        "All",
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
        5 => samplers_real_quality_benchmark(),
        6 => demo_algorithms(),
        7 => demo_samplers(),
        8 => {
            processing_rate_benchmark();
            dimentionality_processing_time_benchmark();
            real_quality_benchmark();
            synthetic_quality_benchmark();
            processing_rate_samplers_benchmark();
            samplers_real_quality_benchmark();
            demo_algorithms();
            demo_samplers();
        }
        _ => (),
    }
}
