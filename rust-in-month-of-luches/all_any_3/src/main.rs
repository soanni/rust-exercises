fn main() {
    let mut v = vec![6; 1000];
    v.push(5);
    let mut cycle = 0;
    let mut i = v.iter();

    loop {
        cycle += 1;

        if i.next() == Some(&5) {
            break;
        }
    }

    println!("{cycle} cycles passed");
}
