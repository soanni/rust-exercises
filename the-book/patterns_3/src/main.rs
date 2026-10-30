fn main() {
    let v = vec![1, 2, 3, 4];

    for (k, v) in v.iter().enumerate() {
        println!("{k} -> {v}");
    }
}
