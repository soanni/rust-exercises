fn main() {
    let v = vec![1, 2, 3, 4];
    let vv = v
        .iter()
        .inspect(|x| println!("item is {x}"))
        .map(|x| x * 2)
        .inspect(|x| println!("then item is {x}"))
        .collect::<Vec<_>>();
}
