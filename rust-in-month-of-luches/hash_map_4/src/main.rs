use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    let key = 1;
    println!("{:p}", &key);

    hm.insert(key, "Samara");

    match hm.get(&key) {
        Some(s) => println!("Existing key is {s}"),
        None => {
            hm.insert(key, "Moscow");
        }
    }
}
