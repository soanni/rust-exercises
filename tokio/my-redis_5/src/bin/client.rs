use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

//#[tokio::main]
//async fn main() {
//    let conn = client::connect("127.0.0.1:6379").await.unwrap();
//
//    let t1 = tokio::spawn(async {
//        conn.set("foo", "bar".into()).await.unwrap();
//    });
//
//    let t2 = tokio::spawn(async {
//        conn.get("foo").await.unwrap();
//    });
//
//    t1.await.unwrap();
//    t2.await.unwrap();
//}
//

enum Command {
    Get { key: String },

    Set { key: String, value: Bytes },
}

/// When every Sender has gone out of scope or has otherwise been dropped, it is no longer possible to send more messages into the channel. At this point, the recv call on the Receiver will return None, which means that all senders are gone and the channel is closed.

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let tx2 = tx.clone();

    // endless while loope as channel is never closed
    // let tx3 = tx.clone();

    let _t1 = tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    let _t2 = tokio::spawn(async move {
        tx2.send("world").await.unwrap();
    });

    while let Some(v) = rx.recv().await {
        println!("{}", v);
    }
}
