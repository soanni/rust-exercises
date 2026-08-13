fn print_country(country: String) -> String {
    println!("{country}");
    country
}

fn main() {
    let c = String::from("Russia");
    let c = print_country(c);
    print_country(c);
}
