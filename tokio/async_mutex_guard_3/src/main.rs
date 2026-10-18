use std::{sync::Mutex, time::Duration};

#[derive(Default)]
struct CanIncrement {
    m: Mutex<i32>,
}

impl CanIncrement {
    fn increment(&self) {
        let mut mg = self.m.lock().unwrap();
        *mg += 1;
    }
}

#[tokio::main]
async fn main() {
    let can = CanIncrement { m: Mutex::new(1) };
    tokio::task::spawn(async move {
        increment_and_do_stuff(&can).await;
    });
    //println!("{:?}", m);
}

async fn increment_and_do_stuff(m: &CanIncrement) {
    m.increment();
    do_smth().await;
}

async fn do_smth() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
