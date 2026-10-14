use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();
    let _ = reqwest::get(url).await?;
    let elapsed = start_time.elapsed();
    println!(
        "It took {} millis to make a request to URL {}",
        elapsed.as_millis(),
        url
    );
    Ok(())
}
