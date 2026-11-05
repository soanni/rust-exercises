fn main() {
    let s = Some(String::from("some awesome string"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");
}
