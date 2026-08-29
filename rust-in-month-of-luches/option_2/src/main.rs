fn take_5th_value(arr: Vec<i32>) -> Option<i32> {
    if arr.len() >= 5 {
        Some(arr[4])
    } else {
        None
    }
}

fn handle_options(arr: &Vec<Option<i32>>) {
    for i in arr {
        match i {
            Some(num) => println!("Found {num}"),
            None => println!("Found None"),
        }
    }
}

fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    let mut options = Vec::new();
    options.push(take_5th_value(small));
    options.push(take_5th_value(big));
    handle_options(&options);
}
