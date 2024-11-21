use super::algorithms::{
    birch::Birch, clustream::CluStream, denstream::Denstream, stream::Stream,
    DataStreamClusteringAlgorithm,
};
use super::samplers::{dynamic_sampler::DynamicSampler, uniform_sampler::UniformSampler, Sampler};

use csv::ReaderBuilder;
use std::fs::File;

const DATA_SETS: [&str; 2] = [
    "benchmark_data/demo.csv",
    "benchmark_data/synthetic/random_5k_2d.csv",
];

type AlorithmFactory = Box<dyn Fn() -> Box<dyn DataStreamClusteringAlgorithm>>;
type SamplerFactory = Box<dyn Fn(Box<dyn DataStreamClusteringAlgorithm>) -> Box<dyn Sampler>>;

pub fn benchmark_algorithms() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        // Box::new(|| Box::new(BIRCH::new(5., 2))),
        Box::new(|| Box::new(Stream::new(5))),
        //Box::new(|| Box::new(CluStream::new())),
        Box::new(|| Box::new(Denstream::new())),
    ];

    for data_set in DATA_SETS.iter() {
        for factory in algorithm_factories.iter() {
            let file = File::open(data_set).unwrap();
            let mut rdr = ReaderBuilder::new().from_reader(file);
            let mut algorithm = factory();
            println!(
                "AlgoBenchmark(Algorithm: {:?} Dataset: {:?})",
                algorithm.name(),
                data_set
            );

            for result in rdr.records() {
                let record = result.unwrap();
                let data: Vec<f64> = record.iter().map(|s| s.parse().unwrap()).collect();
                algorithm.insert(data.clone());
            }

            drop(algorithm);
        }
    }
}

pub fn benchmark_algorithms_with_samplers() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Stream::new(5))),
        // Box::new(|| Box::new(Birch::new(5., 2))),
        Box::new(|| Box::new(CluStream::new())),
        Box::new(|| Box::new(Denstream::new())),
    ];

    let sampler_factories: Vec<SamplerFactory> = vec![
        Box::new(|algorithm| Box::new(UniformSampler::new(algorithm))),
        Box::new(|algorithm| Box::new(DynamicSampler::new(algorithm))),
    ];

    for data_set in DATA_SETS.iter() {
        let file = File::open(data_set).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for factory in algorithm_factories.iter() {
            for sampler_factory in sampler_factories.iter() {
                let algorithm = factory();
                let algo_name = algorithm.name();
                let mut sampler = sampler_factory(algorithm);
                println!(
                    "SamplerBenchmark(Sampler: {:?}, Algorithm: {:?}, Dataset: {:?})",
                    sampler.name(),
                    algo_name,
                    data_set
                );

                for result in rdr.records() {
                    let record: Vec<f64> =
                        result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
                    let data = record[..record.len() - 1].to_vec();
                    let _ = record.last().unwrap();

                    sampler.insert(data.clone());
                }

                drop(sampler);
            }
        }
    }
}
