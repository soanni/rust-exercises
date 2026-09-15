fn main() {
    let my_vec = vec![8, 9, 10];

    let val = my_vec.get(3).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            val
        } else {
            &0
        }
    });
    println!("{val}");
}
