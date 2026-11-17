use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let samara_name = Rc::new("Samara".to_string());
    let samara_history = Rc::new("Samara was founded in 1586 ...".to_string());

    let samara = City {
        name: Rc::clone(&samara_name),
        population: 1_200_000,
        history: Rc::clone(&samara_history),
    };

    let histories = CityData {
        names: vec![Rc::clone(&samara_name)],
        histories: vec![Rc::clone(&samara_history)],
    };

    println!("{}", samara_name);
    println!(
        "number of strong_count for samara_name is {}",
        Rc::strong_count(&samara_name)
    );
}
