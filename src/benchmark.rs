use super::algorithms::{
    birch::Birch, clustream::CluStream, denstream::Denstream, ssq, DataStreamClusteringAlgorithm,
};
use super::samplers::{
    kmeans_dynamic_sampler::KMeansDynamicSampler, static_sampler::StaticSampler, Sampler,
};

use csv::{ReaderBuilder, Writer};
use std::fs::File;
use std::time::Instant;

const PROCESSING_RATE_DATASETS: [&str; 1] = ["benchmark_data/synthetic/random_5k_4d.csv"];

const DIMENTIONALITY_DATA_SETS: [&str; 8] = [
    "benchmark_data/synthetic/random_5k_2d.csv",
    "benchmark_data/synthetic/random_5k_4d.csv",
    "benchmark_data/synthetic/random_5k_5d.csv",
    "benchmark_data/synthetic/random_5k_10d.csv",
    "benchmark_data/synthetic/random_5k_20d.csv",
    "benchmark_data/synthetic/random_5k_40d.csv",
    "benchmark_data/synthetic/random_5k_60d.csv",
    "benchmark_data/synthetic/random_5k_80d.csv",
];

type AlorithmFactory = Box<dyn Fn() -> Box<dyn DataStreamClusteringAlgorithm>>;
type SamplerFactory = Box<dyn Fn(Box<dyn DataStreamClusteringAlgorithm>) -> Box<dyn Sampler>>;

pub fn processing_rate_benchmark() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(2., 50, 5))),
        Box::new(|| Box::new(CluStream::new(5))),
        Box::new(|| Box::new(Denstream::new())),
    ];

    // Processing rate benchmark
    let d = [4];
    let processing_rate_file = File::create("./benchmark_results/processing_rate.csv").unwrap();
    let mut writer = Writer::from_writer(processing_rate_file);
    writer
        .write_record(["algorithm", "dimention", "interval", "record_no"])
        .unwrap();
    for (d_idx, data_set) in PROCESSING_RATE_DATASETS.iter().enumerate() {
        for _ in 0..8 {
            for factory in algorithm_factories.iter() {
                let data_file = File::open(data_set).unwrap();
                let mut rdr = ReaderBuilder::new().from_reader(data_file);
                let mut algorithm = factory();
                println!(
                    "ProcessingRateAlgoBenchmark(Algorithm: {:?} Dataset: {:?})",
                    algorithm.name(),
                    data_set
                );
                let mut start = Instant::now();
                let mut results: Vec<(String, String, String, String)> = Vec::new();
                for (record_no, result) in rdr.records().enumerate() {
                    let data: Vec<f64> = result
                        .unwrap()
                        .iter()
                        .take(d[d_idx])
                        .map(|s| s.parse().unwrap())
                        .collect();
                    algorithm.insert(data);
                    if record_no % 10_000 == 0 {
                        results.push((
                            algorithm.name(),
                            d[d_idx].to_string(),
                            start.elapsed().as_micros().to_string(),
                            record_no.to_string(),
                        ));
                        start = Instant::now();
                    }
                }
                results.push((
                    algorithm.name(),
                    d[d_idx].to_string(),
                    start.elapsed().as_micros().to_string(),
                    "1000000".to_string(),
                ));
                for result in results.iter() {
                    writer
                        .write_record(&[&result.0, &result.1, &result.2, &result.3])
                        .unwrap();
                }
                drop(algorithm);
            }
        }
    }
    writer.flush().unwrap();
}

pub fn dimentionality_processing_time_benchmark() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(5., 50, 5))),
        Box::new(|| Box::new(CluStream::new(5))),
        Box::new(|| Box::new(Denstream::new())),
    ];
    // Dimentionality processing rate benchmark
    let d = [2, 4, 5, 10, 20, 40, 60, 80];
    let dimentionality_file =
        File::create("./benchmark_results/dimentionality_processing_time.csv").unwrap();
    let mut writer = Writer::from_writer(dimentionality_file);
    writer
        .write_record(["algorithm", "dimention", "processing_time"])
        .unwrap();
    for (d_idx, data_set) in DIMENTIONALITY_DATA_SETS.iter().enumerate() {
        for factory in algorithm_factories.iter() {
            let data_file = File::open(data_set).unwrap();
            let mut rdr = ReaderBuilder::new().from_reader(data_file);
            let mut algorithm = factory();
            println!(
                "DimentionalityAlgoBenchmark(Algorithm: {:?} Dataset: {:?})",
                algorithm.name(),
                data_set
            );
            let start = Instant::now();
            for result in rdr.records() {
                let data: Vec<f64> = result
                    .unwrap()
                    .iter()
                    .take(d[d_idx])
                    .map(|s| s.parse().unwrap())
                    .collect();
                algorithm.insert(data);
            }
            writer
                .write_record(&[
                    algorithm.name(),
                    d[d_idx].to_string(),
                    start.elapsed().as_secs_f64().to_string(),
                ])
                .unwrap();
            drop(algorithm);
        }
    }
    writer.flush().unwrap();
}

pub fn real_quality_benchmark() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(2., 50, 2))),
        Box::new(|| Box::new(CluStream::new(2))),
        Box::new(|| Box::new(Denstream::new())),
    ];

    let data_set = "benchmark_data/real/converted_RT_IOT2022_10.csv";
    let output_file = File::create("./benchmark_results/RT_IOT2022_quality.csv").unwrap();
    let mut writer = Writer::from_writer(output_file);
    writer.write_record(["algorithm", "SSQ"]).unwrap();
    for factory in algorithm_factories.iter() {
        let mut algorithm = factory();
        // input
        let data_file = File::open(data_set).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(data_file);
        // demo
        let start = Instant::now();
        for result in rdr.records() {
            let record: Vec<f64> = result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
            algorithm.insert(record);
        }
        let clusters = algorithm.clusters();
        let ssq = ssq(&clusters);
        writer
            .write_record(&[algorithm.name(), ssq.to_string()])
            .unwrap();
        println!(
            "DemoAlgoBenchmark(Algorithm: {:?} Dataset: RT_IOT2022, Time: {:?})",
            algorithm.name(),
            start.elapsed()
        );
    }
}
pub fn synthetic_quality_benchmark() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(5., 50, 5))),
        Box::new(|| Box::new(CluStream::new(5))),
        Box::new(|| Box::new(Denstream::new())),
    ];

    let data_set = "benchmark_data/synthetic/random_5k_4d.csv";
    let output_file = File::create("./benchmark_results/synthetic_quality.csv").unwrap();
    let mut writer = Writer::from_writer(output_file);
    writer.write_record(["algorithm", "SSQ"]).unwrap();

    for factory in algorithm_factories.iter() {
        let mut algorithm = factory();
        let data_file = File::open(data_set).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(data_file);

        let start = Instant::now();
        for result in rdr.records() {
            let record: Vec<f64> = result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
            algorithm.insert(record);
        }
        let clusters = algorithm.clusters();
        let ssq = ssq(&clusters);
        writer
            .write_record(&[algorithm.name(), ssq.to_string()])
            .unwrap();
        println!(
            "SyntheticQualityBenchmark(Algorithm: {:?} Dataset: random_5k_4d, Time: {:?})",
            algorithm.name(),
            start.elapsed()
        );
    }
    writer.flush().unwrap();
}

pub fn demo_algorithms() {
    let demo_name = "blobs";
    let data_path = "demos/blobs_demo.csv";
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(1.0, 50, 3))),
        Box::new(|| Box::new(CluStream::new(3))),
        Box::new(|| Box::new(Denstream::new())),
    ];
    for factory in algorithm_factories.iter() {
        let mut algorithm = factory();
        // output
        let output_file = File::create(format!(
            "./demo_results/{}_{}.csv",
            demo_name,
            algorithm.name()
        ))
        .unwrap();
        let mut writer = Writer::from_writer(output_file);
        writer
            .write_record(["center", "radius", "cluster"])
            .unwrap();
        // input
        let data_file = File::open(data_path).unwrap();
        let mut rdr = ReaderBuilder::new().from_reader(data_file);
        // demo
        let start = Instant::now();
        for result in rdr.records() {
            let record: Vec<f64> = result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
            algorithm.insert(record);
        }
        for cluster in algorithm.clusters() {
            writer
                .write_record(&[
                    cluster
                        .center
                        .iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    cluster.radius.to_string(),
                    cluster.cluster.to_string(),
                ])
                .unwrap();
        }
        println!(
            "DemoAlgoBenchmark(Algorithm: {:?} Dataset: {:?}, Time: {:?})",
            algorithm.name(),
            demo_name,
            start.elapsed()
        );
    }

    let demo_names = ["circles", "moons"];
    let data_paths = ["demos/circles_demo.csv", "demos/moon_demo.csv"];
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(1.0, 50, 2))),
        Box::new(|| Box::new(CluStream::new(2))),
        Box::new(|| Box::new(Denstream::new())),
    ];
    for (i, data_path) in data_paths.iter().enumerate() {
        for factory in algorithm_factories.iter() {
            let mut algorithm = factory();
            // output
            let output_file = File::create(format!(
                "./demo_results/{}_{}.csv",
                demo_names[i],
                algorithm.name()
            ))
            .unwrap();
            let mut writer = Writer::from_writer(output_file);
            writer
                .write_record(["center", "radius", "cluster"])
                .unwrap();
            // input
            let data_file = File::open(data_path).unwrap();
            let mut rdr = ReaderBuilder::new().from_reader(data_file);
            // demo
            let start = Instant::now();
            for result in rdr.records() {
                let record: Vec<f64> = result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
                algorithm.insert(record);
            }
            for cluster in algorithm.clusters() {
                writer
                    .write_record(&[
                        cluster
                            .center
                            .iter()
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                        cluster.radius.to_string(),
                        cluster.cluster.to_string(),
                    ])
                    .unwrap();
            }
            println!(
                "DemoAlgoBenchmark(Algorithm: {:?} Dataset: {:?}, Time: {:?})",
                algorithm.name(),
                demo_names[i],
                start.elapsed()
            );
        }
    }
}

pub fn processing_rate_samplers_benchmark() {
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(3., 50, 5))),
        Box::new(|| Box::new(CluStream::new(5))),
        Box::new(|| Box::new(Denstream::new())),
    ];

    let sampler_factories: Vec<SamplerFactory> = vec![
        // Box::new(|algorithm| Box::new(UniformSampler::new(algorithm))),
        Box::new(|algorithm| Box::new(StaticSampler::new(algorithm, 0.3))),
        Box::new(|algorithm| Box::new(KMeansDynamicSampler::new(algorithm, 5))),
    ];

    let d = [4];
    let processing_rate_file =
        File::create("./benchmark_results/processing_rate_samplers.csv").unwrap();
    let mut pr_writer = Writer::from_writer(processing_rate_file);
    pr_writer
        .write_record(["algorithm", "dimention", "interval", "record_no"])
        .unwrap();
    let quality_file = File::create("./benchmark_results/synthetic_quality_samplers.csv").unwrap();
    let mut quality_writer = Writer::from_writer(quality_file);
    quality_writer.write_record(["algorithm", "SSQ"]).unwrap();
    for (d_idx, data_set) in PROCESSING_RATE_DATASETS.iter().enumerate() {
        for _ in 0..2 {
            for factory in algorithm_factories.iter() {
                for sampler_factory in sampler_factories.iter() {
                    let data_file = File::open(data_set).unwrap();
                    let mut rdr = ReaderBuilder::new().from_reader(data_file);
                    let algorithm = factory();
                    let mut sampler = sampler_factory(algorithm);
                    let name = sampler.name();
                    println!(
                        "ProcessingRateSamplerBenchmark(Algorithm: {:?}, Dataset: {:?})",
                        name,
                        data_set
                    );
                    let mut start = Instant::now();
                    let mut results: Vec<(String, String, String, String)> = Vec::new();
                    for (record_no, result) in rdr.records().enumerate() {
                        let data: Vec<f64> = result
                            .unwrap()
                            .iter()
                            .take(d[d_idx])
                            .map(|s| s.parse().unwrap())
                            .collect();
                        sampler.insert(data);
                        if record_no % 10_000 == 0 {
                            results.push((
                                name.clone(),
                                d[d_idx].to_string(),
                                start.elapsed().as_micros().to_string(),
                                record_no.to_string(),
                            ));
                            start = Instant::now();
                        }
                    }
                    results.push((
                        name.clone(),
                        d[d_idx].to_string(),
                        start.elapsed().as_micros().to_string(),
                        "1000000".to_string(),
                    ));
                    for result in results.iter() {
                        pr_writer
                            .write_record(&[&result.0, &result.1, &result.2, &result.3])
                            .unwrap();
                    }
                    let clusters = sampler.clusters();
                    let ssq = ssq(&clusters);
                    quality_writer
                        .write_record(&[name.as_str(), ssq.to_string().as_str()])
                        .unwrap();
                    drop(sampler);
                }
            }
        }
    }
    pr_writer.flush().unwrap();
    quality_writer.flush().unwrap();
}

pub fn demo_samplers() {
    let demo_name = "blobs";
    let data_path = "demos/blobs_demo.csv";
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(1.0, 50, 3))),
        Box::new(|| Box::new(CluStream::new(3))),
        Box::new(|| Box::new(Denstream::new())),
    ];
    let sampler_factories: Vec<SamplerFactory> = vec![
        // Box::new(|algorithm| Box::new(UniformSampler::new(algorithm))),
        Box::new(|algorithm| Box::new(StaticSampler::new(algorithm, 0.3))),
        Box::new(|algorithm| Box::new(KMeansDynamicSampler::new(algorithm, 3))),
    ];
    for factory in algorithm_factories.iter() {
        for sampler_factory in sampler_factories.iter() {
            let algorithm = factory();
            let mut sampler = sampler_factory(algorithm);
            let name = sampler.name();
            // output
            let output_file = File::create(format!(
                "./demo_results/{}_{}.csv",
                demo_name,
                name
            ))
            .unwrap();
            let mut writer = Writer::from_writer(output_file);
            writer
                .write_record(["center", "radius", "cluster"])
                .unwrap();
            // input
            let data_file = File::open(data_path).unwrap();
            let mut rdr = ReaderBuilder::new().from_reader(data_file);
            // demo
            let start = Instant::now();
            for result in rdr.records() {
                let record: Vec<f64> = result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
                sampler.insert(record);
            }
            for cluster in sampler.clusters() {
                writer
                    .write_record(&[
                        cluster
                            .center
                            .iter()
                            .map(|f| f.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                        cluster.radius.to_string(),
                        cluster.cluster.to_string(),
                    ])
                    .unwrap();
            }
            println!(
                "DemoSamplerBenchmark(Sampler: {:?} Dataset: {:?}, Time: {:?})",
                name,
                demo_name,
                start.elapsed()
            );
        }
    }
    let algorithm_factories: Vec<AlorithmFactory> = vec![
        Box::new(|| Box::new(Birch::new(1.0, 50, 2))),
        Box::new(|| Box::new(CluStream::new(2))),
        Box::new(|| Box::new(Denstream::new())),
    ];
    let sampler_factories: Vec<SamplerFactory> = vec![
        // Box::new(|algorithm| Box::new(UniformSampler::new(algorithm))),
        Box::new(|algorithm| Box::new(StaticSampler::new(algorithm, 0.3))),
        Box::new(|algorithm| Box::new(KMeansDynamicSampler::new(algorithm, 2))),
    ];
    let demo_names = ["circles", "moons"];
    let data_paths = ["demos/circles_demo.csv", "demos/moon_demo.csv"];
    for (i, data_path) in data_paths.iter().enumerate() {
        for factory in algorithm_factories.iter() {
            for sampler_factory in sampler_factories.iter() {
                let algorithm = factory();
                let mut sampler = sampler_factory(algorithm);
                let name = sampler.name();
                // output
                let output_file = File::create(format!(
                    "./demo_results/{}_{}.csv",
                    demo_names[i],
                    name
                ))
                .unwrap();
                let mut writer = Writer::from_writer(output_file);
                writer
                    .write_record(["center", "radius", "cluster"])
                    .unwrap();
                // input
                let data_file = File::open(data_path).unwrap();
                let mut rdr = ReaderBuilder::new().from_reader(data_file);
                // demo
                let start = Instant::now();
                for result in rdr.records() {
                    let record: Vec<f64> = result.unwrap().iter().map(|s| s.parse().unwrap()).collect();
                    sampler.insert(record);
                }
                for cluster in sampler.clusters() {
                    writer
                        .write_record(&[
                            cluster
                                .center
                                .iter()
                                .map(|f| f.to_string())
                                .collect::<Vec<String>>()
                                .join(","),
                            cluster.radius.to_string(),
                            cluster.cluster.to_string(),
                        ])
                        .unwrap();
                }
                println!(
                    "DemoSamplerBenchmark(Sampler: {:?} Dataset: {:?}, Time: {:?})",
                    name,
                    demo_names[i],
                    start.elapsed()
                );
            }
        }
    }
}
