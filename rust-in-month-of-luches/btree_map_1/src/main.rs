use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct City {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn main() {
    let mut samara = City {
        name: "Samara".to_string(),
        population: BTreeMap::new(),
    };

    samara.population.insert(1586, 1000);
    samara.population.insert(1986, 888_901);
    samara.population.insert(2026, 1_118_095);

    for (year, head_count) in samara.population {
        println!("year {year} has {head_count} people");
    }
    //println!("{:?}", samara);
}
