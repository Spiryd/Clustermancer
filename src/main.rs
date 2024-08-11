use futures::stream;

mod reference_algorithms;
use reference_algorithms::birch::birch;

#[tokio::main]
async fn main() {
    // Create a simulated data stream of integers
    let data_stream = stream::iter(vec![1, 2, 3, 4, 5]);

    // Process the stream and get the results
    let results = birch(data_stream).await;

    // Output the processed results
    for result in results {
        println!("Processed data: {}", result);
    }
}
