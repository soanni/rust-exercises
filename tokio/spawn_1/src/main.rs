use std::{thread, time::Duration};

/// A Tokio task is an asynchronous green thread. They are created by passing an async block to tokio::spawn.

/// Tasks are the unit of execution managed by the scheduler. Spawning the task submits it to the Tokio scheduler, which then ensures that the task executes when it has work to do. The spawned task may be executed on the same thread as where it was spawned, or it may execute on a different runtime thread. The task can also be moved between threads after being spawned.

/// Tasks in Tokio are very lightweight. Under the hood, they require only a single allocation and 64 bytes of memory. Applications should feel free to spawn thousands, if not millions of tasks.

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("hello from inside the green thread");
        tokio::time::sleep(Duration::from_secs(5)).await;
        "returneeeddd"
    });

    thread::sleep(Duration::from_secs(1));
    println!("hello from main thread");

    let out = handle.await.unwrap();
    println!("{out}");
}
