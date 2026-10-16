use std::thread;
use std::time::{Duration, Instant};

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let start = Instant::now();

    let person_1 = tokio::task::spawn(async {
        let step_1 = make_coffee_mug(1);
        let step_2 = make_coffee(1);
        let step_3 = make_toast(1);
        tokio::join!(step_1, step_2, step_3);
    });

    let person_2 = tokio::task::spawn(async {
        let step_1 = make_coffee_mug(2);
        let step_2 = make_coffee(2);
        let step_3 = make_toast(2);
        tokio::join!(step_1, step_2, step_3);
    });

    let _ = tokio::join!(person_1, person_2);

    let time_passed = start.elapsed();
    println!(
        "Time passed to make a coffee and a toast: {:?}",
        time_passed
    );
}

async fn make_coffee_mug(id: u64) {
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("[{id}] pouring the milk");
    thread::sleep(Duration::from_secs(2));
    println!("[{id}] milk poured");
    println!("[{id}] adding instant coffee granules");
    thread::sleep(Duration::from_secs(1));
    println!("[{id}] instant coffee granules added");
}

async fn make_coffee(id: u64) {
    println!("[{id}] boiling water");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("[{id}] water boiled");
    println!("[{id}] pouring boiled water to the mug");
    thread::sleep(Duration::from_secs(2));
    println!("[{id}] boiled water poured");
}

async fn make_toast(id: u64) {
    println!("[{id}] making a toast");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("[{id}] toast is ready");
    println!("[{id}] buttering the toasted bread");
    thread::sleep(Duration::from_secs(3));
    println!("[{id}] toast is buttered");
}
