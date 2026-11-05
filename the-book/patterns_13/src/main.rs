fn main() {
    let mut val = Some(5);
    let new_val = Some(10);

    match (val, new_val) {
        (Some(_), Some(_)) => println!("can't unset existing val"),
        _ => val = new_val,
    }

    println!("current val is {val:?}");
}
