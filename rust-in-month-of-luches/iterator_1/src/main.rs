fn main() {
    let v = (1..).take(10).collect::<Vec<i32>>();
    println!("{v:?}");
}
