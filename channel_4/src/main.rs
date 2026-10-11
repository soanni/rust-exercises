use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            "hi".to_string(),
            "from".to_string(),
            "the".to_string(),
            "spawned".to_string(),
            "thread".to_string(),
        ];
        msgs.into_iter().for_each(|x| {
            tx1.send(x).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    });

    thread::spawn(move || {
        let msgs = vec![
            "we".to_string(),
            "have".to_string(),
            "more".to_string(),
            "messages".to_string(),
            "for".to_string(),
            "you".to_string(),
        ];
        msgs.into_iter().for_each(|x| {
            tx.send(x).unwrap();
            thread::sleep(Duration::from_secs(1));
        });
    });

    rx.into_iter().for_each(|x| println!("{x}"));
}
