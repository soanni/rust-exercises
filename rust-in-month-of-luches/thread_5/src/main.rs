fn main() {
    let mut handles = vec![];

    for num in 0..10 {
        let h = std::thread::spawn(|| {
            println!("thread #{num}");
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}
