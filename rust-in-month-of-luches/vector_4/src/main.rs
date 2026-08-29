fn main() {
    let mut v = vec![0; 600_000];
    for _ in 0..600_000 {
        v.remove(0);
    }
}
