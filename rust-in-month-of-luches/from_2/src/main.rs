#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl Country {
    fn print_cities(&self) {
        for c in &self.cities {
            println!("{c:?}");
        }
    }
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Country {
        Country { cities }
    }
}

fn main() {
    let samara = City::new("Samara", 1_001_002);
    let moscow = City::new("Moscow", 10_001_002);
    let russia = Country::from(vec![samara, moscow]);
    russia.print_cities();
}
