enum Climate {
    Tropical,
    Continental,
    Polar,
}

struct Country {
    population: u32,
    capital: String,
    leader: String,
    climate: Climate,
}

fn main() {
    let leader = "trump".to_string();
    let population = 400_000_000;
    let capital = "washington".to_string();

    let usa = Country {
        capital,
        leader,
        population,
        climate: Climate::Continental,
    };
}
