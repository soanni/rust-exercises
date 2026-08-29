fn main() {
    let v = vec![0, 1, 2];
    let n = v.get(3).unwrap_or(&0);
    println!("{n}");
}
