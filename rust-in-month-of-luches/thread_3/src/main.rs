fn main() {
    for _ in 0..10 {
        let h = std::thread::spawn(|| {
            println!("smth");
        });
        h.join().unwrap();
    }
}
