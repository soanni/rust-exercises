use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

use crate::Command::Get;

enum Command {
    Get { key: String },

    Set { key: String, value: Bytes },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        let mut conn = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key } => {
                    conn.get(&key).await.unwrap();
                }
                Set { key, value } => {
                    conn.set(&key, value).await.unwrap();
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "foo".to_string(),
        };
        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
        };
        tx2.send(cmd).await.unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
