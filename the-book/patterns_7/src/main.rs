/// matching multiple patterns
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }
}
