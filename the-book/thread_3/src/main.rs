use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let h = thread::spawn(move || {
        println!("here is the vector {v:?}");
    });

    // drop(v);

    h.join().unwrap();
}
