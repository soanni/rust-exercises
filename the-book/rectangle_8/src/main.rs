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
}
