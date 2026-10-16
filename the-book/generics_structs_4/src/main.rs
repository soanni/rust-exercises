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

impl<T, U> Point<T, U> {
    fn mixup<Z, V>(self, other: Point<Z, V>) -> Point<T, V> {
        Point {
            x: self.x,
            y: other.y,
            z: self.z,
        }
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

    let p4 = Point {
        x: "some string".to_string(),
        y: 'c',
        z: 0,
    };

    println!("p1.x = {}", p1.x());
    println!("p1.x = {}", p1.x);
    //    println!("p1.x = {}", p1.x);
    println!("p2 distance_from_origin = {}", p2.distance_from_origin());

    let p5 = p3.mixup(p4);
    println!(
        "p3 mixup: p3.x = {}, p3.y = {}, p3.z = {}",
        p5.x, p5.y, p5.z
    );
}
