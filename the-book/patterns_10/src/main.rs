/// enum destructuring
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    use Message::*;

    let m = Message::ChangeColor(0, 0, 255);

    match m {
        Quit => println!("quit"),
        Move { x, y } => println!("move in x: {x}, y: {y}"),
        Write(s) => println!("Write({s})"),
        ChangeColor(x, y, z) => println!("rgb: {x}, {y}, {z}"),
    }
}
