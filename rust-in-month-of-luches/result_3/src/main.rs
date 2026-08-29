fn check_if_its_five(n: i32) -> Result<i32, String> {
    match n {
        5 => Ok(n),
        _ => Err(format!("Oops not a five, its {n} instead")),
    }
}

fn main() {
    for i in 1..=7 {
        println!("{:?}", check_if_its_five(i));
    }
}
