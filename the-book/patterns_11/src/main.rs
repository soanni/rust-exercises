/// enum destructuring
/// 2 level nesting
///

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    use Color::*;
    use Message::*;

    let m = Message::ChangeColor(Hsv(1, 255, 255));

    match m {
        ChangeColor(Hsv(h, s, v)) => println!("change color to HSV: ({h},{s},{v})"),
        ChangeColor(Rgb(r, g, b)) => println!("change color to RGB: ({r},{g},{b})"),
        _ => (),
    }
}
