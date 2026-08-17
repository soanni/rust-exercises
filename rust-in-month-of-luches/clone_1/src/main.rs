fn print_country(c: String) {
    println!("{c}");
}

fn main() {
    let c = String::from("Russia");
    print_country(c.clone());
    print_country(c);
}
