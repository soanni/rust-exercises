fn print_str(my_str: &str) {
    println!("{my_str}");
}

fn main() {
    let s = String::from("this is a string");
    print_str(&s);
}
