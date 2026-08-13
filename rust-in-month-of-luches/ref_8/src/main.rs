fn print_country(country: &String) {
    println!("{country}");
}

fn add_hungary(country: &mut String) {
    country.push_str("-hungary");
    println!("now it says: {country}");
}

fn main() {
    let mut c = String::from("Russia");
    print_country(&c);
    print_country(&c);
    add_hungary(&mut c);
    println!("{c}");
}
