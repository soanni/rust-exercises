use tokio::net::TcpStream;

async fn my_async() {
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    println!("async tcp connection complete");
}

#[tokio::main]
async fn main() {
    let blabla = my_async();
    blabla.await;
}
