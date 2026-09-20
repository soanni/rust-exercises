#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}

fn main() {
    let c = City {
        name: "Samara",
        date_founded: 1586,
    };

    println!("my city {} wqs founded in {}", c.name, c.date_founded);
}
