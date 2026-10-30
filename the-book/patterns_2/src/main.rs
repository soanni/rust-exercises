use std::{sync::mpsc, thread};

fn main() {
    let (tx, rv) = mpsc::channel();

    thread::spawn(move || {
        let v = vec![1, 2, 3];

        for i in v {
            tx.send(i).unwrap();
        }
    });

    while let Ok(i) = rv.recv() {
        println!("{i}");
    }
}
