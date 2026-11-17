#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    founded: u32,
}

#[derive(Debug)]
struct Country<'a> {
    cities: Vec<City<'a>>,
}

#[derive(Debug)]
struct World<'a> {
    countries: Vec<Country<'a>>,
}

impl World<'_> {}

fn main() {
    let cities = vec!["Samara".to_string(), "Moscow".to_string()];

    let samara = City {
        name: &cities[0],
        founded: 1586,
    };

    println!("{:?}", samara);
}
