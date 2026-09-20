use std::sync::RwLock;

fn main() {
    let l = RwLock::new(5);
    let lr_1 = l.read().unwrap();
    let lr_2 = l.read().unwrap();
    println!("{lr_1}");
    println!("{lr_2}");
    drop(lr_1);
    drop(lr_2);
    {
        let mut w = l.write().unwrap();
        *w = 6;
    }
    println!("{l:?}");
}
