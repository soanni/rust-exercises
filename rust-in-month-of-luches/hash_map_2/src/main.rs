use std::collections::HashMap;

fn main() {
    let russian_cities = vec!["Samara", "Moscow", "Saint Petersburg", "Kemerovo"];
    let us_cities = vec!["New York", "Washington", "Palo Alto", "San Francisco"];

    let mut cities_hash = HashMap::new();

    for c in russian_cities {
        cities_hash.insert(c.to_string(), "Russia");
    }

    for c in us_cities {
        cities_hash.insert(c.to_string(), "USA");
    }

    println!("{:?}", cities_hash["Samara"]);
    println!("{:?}", cities_hash.get("Samara"));
    println!("{:?}", cities_hash.get("Samaraa"));
    //    println!("{:?}", cities_hash["Samaraa"]);
}
