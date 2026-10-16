use std::thread;
use std::time::Instant;

fn fib(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

fn main() {
    let start = Instant::now();

    let mut threads = Vec::new();

    for _ in 0..4 {
        let handle = thread::spawn(|| fib(50));

        threads.push(handle);
    }

    for (i, h) in threads.into_iter().enumerate() {
        let r = h.join();
        match r {
            Ok(r) => println!("thread #{}, fib(50) = {}", i + 1, r),
            Err(e) => eprintln!("thread #{} error happened {:?}", i + 1, e),
        }
    }

    println!(
        "4 threads calculating fib(50) are done in {:?}",
        start.elapsed()
    );
}
