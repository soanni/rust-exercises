//fn returns_ref() -> &str {
//    let s = String::from("this is a string");
//    &s
//}

fn returns_str() -> &'static str {
    let s = String::from("this is a string");
    "this is a string"
}

fn main() {
    let s = returns_str();
    println!("{s}");
}
