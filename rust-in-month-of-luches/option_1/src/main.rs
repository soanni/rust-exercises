fn take_5th_value(arr: Vec<i32>) -> Option<i32> {
    if arr.len() >= 5 {
        Some(arr[4])
    } else {
        None
    }
}

fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    println!(
        "{:?}, {:?}",
        take_5th_value(small).unwrap(),
        take_5th_value(big).unwrap()
    );
}
