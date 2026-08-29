use std::collections::HashSet;

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];

    println!("original vector length is {}", many_numbers.len());

    let mut hash_set = HashSet::new();

    for n in many_numbers {
        hash_set.insert(n);
    }

    println!("hashset length is {}", hash_set.len());

    println!("missing numbers are: ");

    for i in 0..=50 {
        if hash_set.get(&i).is_none() {
            print!("{i} ");
        }
    }
}
