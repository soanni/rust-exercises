fn main() {
    let v = vec![0, 1, 2];
    let mut i = v.into_iter().peekable();

    for _ in 0..3 {
        println!("{} is nice", i.peek().unwrap());
        println!("{} is cool", i.peek().unwrap());
        println!("{} is awesome", i.peek().unwrap());
        i.next();
    }
}
