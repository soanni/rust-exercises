use std::num::ParseIntError;

fn parse_and_log(input: &str) -> Result<i32, ParseIntError> {
    let num = input.parse::<i32>()?;
    println!("The parsed number is {num}");
    Ok(num)
}

fn main() {
    let v = vec!["1", "1.0", "one", "6060"];
    for i in v {
        let p = parse_and_log(i);
        println!("{p:?}");
    }
}
