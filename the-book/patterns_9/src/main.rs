struct Point {
    x: i32,
    y: i32,
}

/// Destructuring and matching literal values in one pattern
fn main() {
    let p = Point { x: 0, y: 11 };

    match p {
        Point { x: 0, y } => println!("on y axis at {y}"),
        Point { x, y: 0 } => println!("on x axis at {x}"),
        Point { x, y } => println!("on neither axis at ({x},{y})"),
    }
}
