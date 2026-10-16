use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = client::connect("localhost:6379").await?;
    conn.set("hello", "world".into()).await?;
    let v = conn.get("hello").await?;
    println!("value is {:?}", v);
    Ok(())
}
