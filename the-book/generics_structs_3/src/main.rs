struct Point<T, U> {
    x: T,
    y: U,
    z: i32,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 1, y: 3, z: 1 };

    let p2 = Point {
        x: 5.0,
        y: 55.5,
        z: -1,
    };

    let p3 = Point {
        x: 1.1,
        y: 1,
        z: -1,
    };

    println!("p1.x = {}", p1.x());
    println!("p1.x = {}", p1.x);
    //    println!("p1.x = {}", p1.x);
    println!("p2 distance_from_origin = {}", p2.distance_from_origin());
}
