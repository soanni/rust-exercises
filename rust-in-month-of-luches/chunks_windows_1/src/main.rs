fn main() {
    let v = vec![0, 1, 2, 3, 4, 5, 6, 7];
    for c in v.chunks(3) {
        println!("{c:?}");
    }

    println!();

    for w in v.windows(3) {
        println!("{w:?}");
    }
}
