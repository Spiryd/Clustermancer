use tokio_stream::StreamExt;
use futures::stream::Stream;

// Define a type alias for a boxed future with a vector of i32

// Function that takes a stream, processes it, and returns a future with the results
pub async fn birch<S>(stream: S) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    // Collect results after doubling each item in the stream
    let results: Vec<i32> = stream
        .map(|data| data * 2)
        .collect()
        .await;

    results
}
