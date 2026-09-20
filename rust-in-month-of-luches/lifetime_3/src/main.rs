#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}

fn main() {
    let my_cities = vec!["Samara".to_string(), "Moscow".to_string()];

    let c = City {
        name: &my_cities[0],
        date_founded: 1586,
    };

    println!("my city {} wqs founded in {}", c.name, c.date_founded);
}
