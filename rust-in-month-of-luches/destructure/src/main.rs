struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: &str, name_before: &str, population: u32, date_founded: u32) -> Self {
        Self {
            population,
            date_founded,
            name: String::from(name),
            name_before: String::from(name_before),
        }
    }

    fn print_city(&self) {
        let City {
            //            population: people,
            //            date_founded: foundation,
            name,
            name_before,
            ..
        } = self;
        println!("It's a city {name} and previous name was {name_before}");
    }
}

fn main() {
    let samara = City::new("Samara", "Kuibyshev", 1_110_997, 1786);
    samara.print_city();
}
