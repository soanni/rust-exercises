use std::collections::HashMap;

fn main() {
    let v1 = vec![0, 1, 2, 3, 4, 5];
    let v2 = vec!["zero", "one", "two", "three", "four", "five"];

    let hm = v1
        .into_iter()
        .zip(v2.into_iter())
        .collect::<HashMap<_, _>>();

    println!("The value for key '2' is {}", hm.get(&2).unwrap());
}
