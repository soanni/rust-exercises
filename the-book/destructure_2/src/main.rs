struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };

    let Point { x, y } = p;

    assert_eq!(1, x);
    assert_eq!(2, y);
}
