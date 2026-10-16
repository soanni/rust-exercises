fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("largest number in list is {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("largest number in list is {}", largest(&number_list));

    let char_list = vec!['a', 'c', 'e', 'm', 'q'];

    println!("largest char in the list is {}", largest(&char_list));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}
