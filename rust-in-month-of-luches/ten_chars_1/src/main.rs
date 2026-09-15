fn main() {
    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let ten_chars_first_skipped = ('a'..).skip(10000).take(10).collect::<Vec<char>>();
    println!("{:?}", ten_chars);
    println!("{:?}", ten_chars_first_skipped);
}
