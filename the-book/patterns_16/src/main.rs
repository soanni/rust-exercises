#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: -1, y: 1, z: 1 };

    match origin {
        Point { x, .. } => println!("x = {x}"),
    }

    println!("{origin:?}");
}
