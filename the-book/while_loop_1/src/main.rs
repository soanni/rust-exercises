fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < a.len() {
        println!("the element at index {} is {}", index, a[index]);
        index += 1;
    }
}
