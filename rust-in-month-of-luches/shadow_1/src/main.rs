fn main() {
    let country = String::from("Russia");
    let country_ref = &country;
    let country = 63;
    println!("{country_ref} {country}");
}
