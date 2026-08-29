fn main() {
    let mut v = vec![0, 1, 2, 3, 4, 5];
    v.pop();
    v.remove(0);
    println!("{:?}", v);
}
