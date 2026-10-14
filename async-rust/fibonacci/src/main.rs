use std::thread;

fn fib(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}

fn main() {
    let mut threads = Vec::new();

    for i in 1..8 {
        let handle = thread::spawn(move || {
            let result = fib(4000);
            println!("thread #{i}, fib(4000) = {result}");
        });

        threads.push(handle);
    }

    for h in threads {
        h.join().unwrap();
    }
}
