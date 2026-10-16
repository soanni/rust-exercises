use std::thread;
use std::time::{Duration, Instant};

/// When we pass our futures into the tokio::join macro, all the async expressions are evaluated concurrently in the same task.
/// The join macro does not create tasks; it merely enables multiple futures to be executed concurrently within the task.

#[tokio::main]
async fn main() {
    let coffee_mug = make_coffee_mug();
    let coffee = make_coffee();
    let toast = make_toast();

    let start = Instant::now();
    tokio::join!(coffee_mug, coffee, toast);
    let time_passed = start.elapsed();
    println!(
        "Time passed to make a coffee and a toast: {:?}",
        time_passed
    );
}

async fn make_coffee_mug() {
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("pouring the milk");
    thread::sleep(Duration::from_secs(2));
    println!("milk poured");
    println!("adding instant coffee granules");
    thread::sleep(Duration::from_secs(1));
    println!("instant coffee granules added");
}

async fn make_coffee() {
    println!("boiling water");
    tokio::time::sleep(Duration::from_secs(10)).await;
    println!("water boiled");
    println!("pouring boiled water to the mug");
    thread::sleep(Duration::from_secs(2));
    println!("boiled water poured");
}

async fn make_toast() {
    println!("making a toast");
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("toast is ready");
    println!("buttering the toasted bread");
    thread::sleep(Duration::from_secs(3));
    println!("toast is buttered");
}
