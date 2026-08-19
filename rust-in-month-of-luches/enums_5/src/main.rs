enum Number {
    U32(u32),
    I32(i32),
}

fn get_number(n: i32) -> Number {
    use Number::*;

    match n.is_positive() {
        true => U32(n as u32),
        false => I32(n),
    }
}

fn main() {
    let arr = vec![get_number(-800), get_number(8)];
    for n in arr {
        match n {
            Number::I32(num) => println!("I32 {num}"),
            Number::U32(num) => println!("U32 {num}"),
        }
    }
}
