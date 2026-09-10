fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    let mut v2 = vec![6, 7, 8, 9, 10];

    for i in v1.iter() {
        println!("{i}");
    }

    for i in v1 {
        println!("{i}");
    }

    for i in v2.iter_mut() {
        *i *= 10;
    }

    println!("{v2:?}");
    //println!("{v1:?}");
}
