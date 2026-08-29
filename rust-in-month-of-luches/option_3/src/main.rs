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
    let supervec = vec![small, big];
    for v in supervec {
        let inside = take_5th_value(v);
        if inside.is_some() {
            println!("we got {}", inside.unwrap());
        } else {
            println!("we got nothing");
        }
    }
}
