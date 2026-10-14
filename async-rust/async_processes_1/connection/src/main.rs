use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Body is {}", body);
    } else {
        println!("Failed to get {}, status is {}", url, response.status());
    }

    Ok(())
}
