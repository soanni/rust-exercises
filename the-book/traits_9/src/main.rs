use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("the largest member is x");
        } else {
            println!("the largest member is y");
        }
    }
}

fn main() {
    let p = Pair::new(1, 3);
    p.cmp_display();
}
