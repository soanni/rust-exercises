use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();
    hm.insert(1, 11);
    hm.insert(1, 12);
    hm.insert(1, 13);

    if let Some(n) = hm.get(&1) {
        println!("{:p}", n);
    }
    //println!("{:?}", hm.get(&1));
}
