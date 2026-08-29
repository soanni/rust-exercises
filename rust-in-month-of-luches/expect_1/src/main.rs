fn get_fourth(v: &Vec<i32>) -> i32 {
    let n = v
        .get(3)
        .expect("The input vector must be at least 4 element long");
    *n
}

fn main() {
    let v = vec![0, 1, 2];
    get_fourth(&v);
}
