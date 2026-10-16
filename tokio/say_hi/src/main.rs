use std::time::Duration;

async fn say_hi() {
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("world");
}

#[tokio::main]
async fn main() {
    let op = say_hi();
    op.await;
    println!("hello");
}
