fn main() {
    let v1 = vec![1, 2, 3];
    let mut v2 = vec![10, 20, 30];

    for e in v1.iter() {
        println!("v1 ref Item is &i32: {e}");
    }

    for e in v1.into_iter() {
        println!("v1 Item is i32: {e}");
    }

    for e in v2.iter_mut() {
        *e *= 10;
        println!("v2 mut item is: {e}");
    }

    //println!("{v1:?}");
    println!("{v2:?}");
}
