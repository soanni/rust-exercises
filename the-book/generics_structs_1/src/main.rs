struct Point<T> {
    x: T,
    y: T,
    z: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 3, z: 1 };

    let p2 = Point {
        x: 5.0,
        y: 55.5,
        z: -1,
    };

    // won't compile
    //
    //let p3 = Point{
    //    x: 1.1,
    //    y: 1,
    //    z: -1
    //}
}
