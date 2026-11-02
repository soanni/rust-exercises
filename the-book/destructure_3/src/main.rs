struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ((a, b), Point { x, y }) = ((3, 5), Point { x: 55, y: 66 });

    println!("a = {a}, b = {b}, x = {x}, y = {y}");
}
