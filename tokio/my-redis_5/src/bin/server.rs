use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use bytes::Bytes;
use mini_redis::Connection;
use mini_redis::Frame;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::sync::Arc;

/// We want our Redis server to process many concurrent requests. To do this, we need to add some concurrency.
/// To process connections concurrently, a new task is spawned for each inbound connection. The connection is processed on this task.

// for convenience, add the following type alias
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("localhost:6379").await.unwrap();

    let db: Arc<Mutex<HashMap<String, Bytes>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, addr) = tcp_listener.accept().await.unwrap();
        println!("new Redis client {:?}", addr);

        let db = db.clone();

        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

//async fn process_socket(socket: TcpStream) {
//    tokio::time::sleep(Duration::from_secs(10)).await;
//    let mut conn = Connection::new(socket);
//
//    if let Some(f) = conn.read_frame().await.unwrap() {
//        println!("GOT {:?}", f);
//        let response = Frame::Error("unimplemented".to_string());
//        conn.write_frame(&response).await.unwrap();
//    }
//}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut conn = Connection::new(socket);

    while let Some(f) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(f).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(v) = db.get(cmd.key()) {
                    Frame::Bulk(v.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };
        conn.write_frame(&response).await.unwrap();
    }
}
