fn turn_into_string_and_parse(input: Vec<u8>) -> i32 {
    let strr = String::from_utf8(input).unwrap();
    let num = strr.parse::<i32>().unwrap();
    num
}

fn main() {
    let n = turn_into_string_and_parse(vec![49, 53, 53]);
    println!("{n}");
}
