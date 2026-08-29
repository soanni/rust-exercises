use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let v = vec!["1", "6060", "1.0", "11", "qwerty"];
    for i in v {
        let n = i.parse::<i32>()?;
        println!("{n}");
    }
    Ok(())
}
