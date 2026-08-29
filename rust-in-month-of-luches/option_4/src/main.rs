fn main() {
    let v = vec![1, 2, 3];

    for i in 0..=10 {
        match v.get(i) {
            Some(n) => println!("{n}"),
            None => {}
        }
    }
}
