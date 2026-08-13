fn print_country(country: String) {
    println!("{country}");
}

fn main() {
    let c = String::from("Russia");
    print_country(c);
    print_country(c);
}
