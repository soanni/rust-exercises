fn main() {
    let v = vec!['x', 'y', 'z'];
    v.iter()
        .enumerate()
        .for_each(|(i, j)| println!("Item {i} is {j}"));
}
