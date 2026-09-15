fn main() {
    for i in ["0", "9.9", "nine", "nine-nine", "8.88"]
        .into_iter()
        .map(|n| n.parse::<f32>())
        .flatten()
    {
        println!("{i:?}");
    }
}
