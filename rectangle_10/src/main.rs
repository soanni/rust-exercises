#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rectangle is {rect:#?}");
    dbg!(&rect);
    println!("the area is {}", rect.area());

    if rect.width() {
        println!(
            "rectangle {rect:?} has a non-zero width and width is {}",
            rect.width
        );
    }

    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 70,
        height: 50,
    };

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    let sq = Rectangle::square(10);

    //dbg!(Rectangle::square(10));

    dbg!(sq);
}
