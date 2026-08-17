fn main() {
    let my_num: u8 = 5;
    match my_num {
        1 => println!("it's one!"),
        2 => println!("it's two!"),
        0 | 3u8..=u8::MAX => println!("smth diff"),
    }
}
