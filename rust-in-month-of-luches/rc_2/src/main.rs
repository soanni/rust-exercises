use std::rc::{Rc, Weak};

#[derive(Debug)]
struct City {
    name: Weak<String>,
    population: u32,
    history: Weak<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<Weak<String>>,
    histories: Vec<Weak<String>>,
}

fn main() {
    let samara_name = Rc::new("Samara".to_string());
    let samara_history = Rc::new("Samara was founded in 1586 ...".to_string());

    let samara = City {
        name: Rc::downgrade(&samara_name),
        population: 1_200_000,
        history: Rc::downgrade(&samara_history),
    };

    let histories = CityData {
        names: vec![Rc::downgrade(&samara_name)],
        histories: vec![Rc::downgrade(&samara_history)],
    };

    println!("{}", samara_name);
    println!(
        "number of strong_count for samara_name is {}",
        Rc::strong_count(&samara_name)
    );

    println!(
        "number of weak refs for samara_name is {}",
        Rc::weak_count(&samara_name)
    );
}
