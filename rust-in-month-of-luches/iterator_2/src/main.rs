fn main() {
    let v: Vec<i32> = (0..10).collect();
    println!("{v:?}");
    //let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vv = v.into_iter().skip(3).take(4).collect::<Vec<i32>>();
    println!("{vv:?}");
}
