use super::algorithms::{DataStreamClusteringAlgorithm, birch::BIRCH, clustream::CluStream, stream::STREAM};
use super::samplers::{Sampler, uniform_sampler::UniformSampler, dynamic_sampler::DynamicSampler};

use csv::ReaderBuilder;
use std::fs::File;

const DATA_SETS: [&str; 1] = ["benchmark_data/demo.csv"];

pub fn benchmark_algorithms() {
    let algorithm_factories: Vec<Box<dyn Fn() -> Box<dyn DataStreamClusteringAlgorithm>>> = vec![
        // Box::new(|| Box::new(BIRCH::new(5., 2))),
        Box::new(|| Box::new(CluStream::new())),
        Box::new(|| Box::new(STREAM::new())),
    ];

    for data_set in DATA_SETS.iter() {
        let file = File::open(data_set).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for factory in algorithm_factories.iter() {
            let mut algorithm = factory();
            println!("AlgoBenchmark(Algorithm: {:?} Dataset: {:?})", algorithm.name(), data_set);

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
    let algorithm_factories: Vec<Box<dyn Fn() -> Box<dyn DataStreamClusteringAlgorithm>>> = vec![
        // Box::new(|| Box::new(Birch::new(5., 2))),
        Box::new(|| Box::new(CluStream::new())),
        Box::new(|| Box::new(STREAM::new())),
    ];

    let sampler_factories: Vec<Box<dyn Fn(Box<dyn DataStreamClusteringAlgorithm>) -> Box<dyn Sampler>>> = vec![
        Box::new(|algorithm| Box::new(UniformSampler::new(algorithm))),
        // Box::new(|algorithm| Box::new(DynamicSampler::new(algorithm))),
    ];

    for data_set in DATA_SETS.iter() {
        let file = File::open(data_set).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(file);

        for factory in algorithm_factories.iter() {
            for sampler_factory in sampler_factories.iter() {
                let algorithm = factory();
                let algo_name = algorithm.name();
                let mut sampler = sampler_factory(algorithm);
                println!("SamplerBenchmark(Sampler: {:?}, Algorithm: {:?}, Dataset: {:?})", sampler.name(), algo_name, data_set);

                for result in rdr.records() {
                    let record = result.unwrap();
                    let data: Vec<f64> = record.iter().map(|s| s.parse().unwrap()).collect();
                    sampler.insert(data.clone());
                }

                drop(sampler);
            }
        }
    }
}