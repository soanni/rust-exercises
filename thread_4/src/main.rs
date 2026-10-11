use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // here Rust infers that v is not borrowed but moved inside the closure
    let h = thread::spawn(|| {
        let vv = v;
        println!("here is the vector {vv:?}");
    });

    // drop(v);

    h.join().unwrap();
}
