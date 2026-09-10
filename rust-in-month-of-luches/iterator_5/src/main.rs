fn main() {
    let v1 = vec![1, 2, 3];
    let mut v2 = vec![10, 20, 30];

    let v1_a = v1.iter().map(|x| *x + 1).collect::<Vec<i32>>();
    let v1_b = v1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

    //for e in v2 {
    //    println!("{e}");
    //}

    v2.iter_mut().for_each(|x| *x += 100);

    //println!("{v1:?}");
    println!("{v1_a:?}");
    println!("{v1_b:?}");
    println!("{v2:?}");
}
