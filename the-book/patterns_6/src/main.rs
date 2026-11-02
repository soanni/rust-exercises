/// matching named variables
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("x = 50"),
        Some(y) => println!("y = {y}"),
        _ => println!("x = {x:?}"),
    }

    println!("x = {x:?}, y = {y}");
}
