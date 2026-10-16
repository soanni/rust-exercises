struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area is {}", area(&rect));
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
