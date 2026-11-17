fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        let h = std::thread::spawn(|| {
            println!("smth");
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
}
