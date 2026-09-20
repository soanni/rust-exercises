#[derive(Debug)]
struct City {
    name: &str,
    date_founded: u32,
}

fn main() {
    let c = City {
        name: "Samara",
        date_founded: 1586,
    };
}
