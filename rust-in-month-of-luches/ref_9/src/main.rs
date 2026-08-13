// TODO: PUT on the site

fn print_country(country: &String) {
    println!("{country}");
}

fn add_hungary(mut country: String) {
    country.push_str("-hungary");
    println!("now it says: {country}");
}

fn main() {
    let c = String::from("Russia");
    print_country(&c);
    print_country(&c);
    add_hungary(c);
}
