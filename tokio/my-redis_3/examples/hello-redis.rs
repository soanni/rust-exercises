use bytes::Bytes;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut conn = match client::connect("localhost:6379").await {
        Ok(c) => c,
        Err(e) => panic!("{e}"),
    };
    conn.set("hello", "world".into()).await?;
    let v = match conn.get("hello").await {
        Ok(r) => match r {
            Some(b) => b,
            None => Bytes::from(""),
        },
        Err(e) => panic!("{e}"),
    };
    println!("value is {:?}", v);
    Ok(())
}
