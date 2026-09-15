fn main() {
    let v = vec![2, 3, 4];
    let vv = v.iter().map(|num| num * 2).collect::<Vec<i32>>();
    println!("{v:?}");
    println!("{vv:?}");
    v.iter()
        .enumerate()
        .map(|(i, j)| println!("item {i} is {j}"));
}
