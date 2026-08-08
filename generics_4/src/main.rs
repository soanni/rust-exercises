fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("largest number in list is {}", largest_i32(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    println!("largest number in list is {}", largest_i32(&number_list));

    let char_list = vec!['a', 'c', 'e', 'm', 'q'];

    println!("largest char in the list is {}", largest_char(&char_list));
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}
