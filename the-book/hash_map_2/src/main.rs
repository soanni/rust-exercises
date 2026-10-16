use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();

    let text = "hello world beautiful world";

    for word in text.split_whitespace() {
        let count = m.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{m:?}");
}
