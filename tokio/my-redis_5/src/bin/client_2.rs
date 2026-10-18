use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },

    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
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
                Get { key, resp } => {
                    let v = conn.get(&key).await;
                    let _ = resp.send(v);
                }
                Set { key, value, resp } => {
                    let v = conn.set(&key, value).await;
                    let _ = resp.send(v);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (ttx, rrx) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: ttx,
        };
        tx.send(cmd).await.unwrap();

        let v = rrx.await;
        println!("Got {:?}", v);
    });

    let t2 = tokio::spawn(async move {
        let (ttx, rrx) = oneshot::channel();

        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: ttx,
        };

        tx2.send(cmd).await.unwrap();

        let v = rrx.await;
        println!("Got {:?}", v);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
