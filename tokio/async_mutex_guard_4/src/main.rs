use std::time::Duration;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let m = Mutex::new(1);
    tokio::task::spawn(async move {
        increment_and_do_stuff(&m).await;
    });
    //println!("{:?}", m);
}

async fn increment_and_do_stuff(m: &Mutex<i32>) {
    let mut mg = m.lock().await;
    *mg += 1;
    do_smth().await;
}

async fn do_smth() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
