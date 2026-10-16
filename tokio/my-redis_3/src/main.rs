use std::time::Duration;

use mini_redis::Connection;
use mini_redis::Frame;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

/// We want our Redis server to process many concurrent requests. To do this, we need to add some concurrency.
/// To process connections concurrently, a new task is spawned for each inbound connection. The connection is processed on this task.

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("localhost:6379").await.unwrap();

    loop {
        let (socket, addr) = tcp_listener.accept().await.unwrap();
        println!("new Redis client {:?}", addr);
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process_socket(socket).await;
        });
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
