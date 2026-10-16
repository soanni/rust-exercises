use reqwest::Error;
use serde::Deserialize;
use serde_json;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
struct Response {
    url: String,
    args: serde_json::Value,
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let data = fetch_data(5);
    let last_login_time = calculate_last_login();
    let (posts, _) = tokio::join!(data, last_login_time);
    let time_passed = start.elapsed();
    println!("Fetched: {:?}", posts);
    println!("Time lasted: {:?}", time_passed);
}

async fn fetch_data(seconds: u64) -> Result<Response, Error> {
    let url = format!("https://httpbin.org/delay/{}", seconds);
    let resp = reqwest::get(&url).await?;
    let resp_json = resp.json().await?;
    Ok(resp_json)
}

async fn calculate_last_login() {
    sleep(Duration::from_secs(1)).await;
    println!("Logged in 7 days ago");
}
