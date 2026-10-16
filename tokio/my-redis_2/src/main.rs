use std::time::Duration;

use mini_redis::Connection;
use mini_redis::Frame;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("localhost:6379").await.unwrap();

    loop {
        let (socket, addr) = tcp_listener.accept().await.unwrap();
        println!("new Redis client {:?}", addr);
        process_socket(socket).await;
    }
}

async fn process_socket(socket: TcpStream) {
    tokio::time::sleep(Duration::from_secs(10)).await;
    let mut conn = Connection::new(socket);
    // let frame = conn.read_frame().await.unwrap();

    if let Some(f) = conn.read_frame().await.unwrap() {
        println!("GOT {:?}", f);
        let response = Frame::Error("unimplemented".to_string());
        conn.write_frame(&response).await.unwrap();
    }
}
