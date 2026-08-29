use std::collections::HashMap;

fn main() {
    let mut books_hash_map = HashMap::new();
    let mut old_hashmap_values = Vec::new();

    let hashmap_k_v = [(1, "value1"), (1, "value2"), (1, "value3"), (1, "value4")];

    for (k, v) in hashmap_k_v {
        if let Some(oldie) = books_hash_map.insert(k, v) {
            old_hashmap_values.push(oldie);
            println!("overwriting {oldie} with {v}");
        }
    }

    println!("{:?}", old_hashmap_values);
}
