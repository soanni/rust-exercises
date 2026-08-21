use std::fmt::Debug;

#[derive(Debug)]
struct Animal {
    age: u8,
    name: String,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item {item:?}");
}

fn main() {
    print_item(5);
    let barsik = Animal {
        age: 8,
        name: "Barsik".to_string(),
    };
    print_item(barsik);
}
