mod algorithms;
mod samplers;
mod benchmark;
use benchmark::{benchmark_algorithms, benchmark_algorithms_with_samplers};

fn main() {
    benchmark_algorithms();
    benchmark_algorithms_with_samplers();
}
