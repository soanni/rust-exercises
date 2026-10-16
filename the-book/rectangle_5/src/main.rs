#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rectangle is {rect:#?}");
    dbg!(&rect);
    println!("the area is {}", area(&rect));
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
