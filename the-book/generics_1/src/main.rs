fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];

    for num in &number_list {
        if num > largest {
            largest = num;
        }
    }

    println!("largest number in list is {largest}");
}
