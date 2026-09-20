use std::sync::RwLock;

fn main() {
    let l = RwLock::new(5);
    let lr_1 = l.try_read().unwrap();
    let lr_2 = l.try_read().unwrap();
    println!("{lr_1}");
    println!("{lr_2}");
    drop(lr_1);
    drop(lr_2);
    if let Ok(mut w) = l.try_write() {
        *w = 6;
    } else {
        println!("couldn't get the write access, sorry ...");
    }
    println!("{l:?}");
}
