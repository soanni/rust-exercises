fn main() {
    let v = vec![1, 2, 3, 4];
    let vv = v
        .iter()
        .inspect(|x| {
            println!("item is {x}");
            println!("item binary is {x:b}");
            match **x % 2 {
                0 => println!("item is even"),
                1 => println!("item is odd"),
                _ => unreachable!(),
            }
        })
        .map(|x| x * 2)
        .inspect(|x| {
            println!("item is {x}");
            println!("item binary is {x:b}");
            match *x % 2 {
                0 => println!("item is even"),
                1 => println!("item is odd"),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
}
