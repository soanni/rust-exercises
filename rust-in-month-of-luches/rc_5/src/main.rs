use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: Rc<String>,
    founded: u32,
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

#[derive(Debug)]
struct World {
    countries: Vec<Country>,
}

impl World {}

fn main() {
    let cities = vec![Rc::new("Samara".to_string()), Rc::new("Moscow".to_string())];

    let samara = City {
        name: Rc::clone(&cities[0]),
        founded: 1586,
    };

    println!("{:?}", samara);
}
