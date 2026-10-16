use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received from the spawned thread: {}", received);
}
