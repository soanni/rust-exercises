fn main() {
    let x = Some(4);

    match x {
        Some(x) if x % 2 == 0 => println!("{x} number is even"),
        Some(x) => println!("{x} number is odd"),
        _ => println!("smth else"),
    }
}
