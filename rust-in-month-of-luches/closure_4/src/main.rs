fn main() {
    (0..=3).for_each(|x| println!("{x}"));
    (0..=3).for_each(|x| {
        println!("Got {x}");
        if x % 2 == 0 {
            println!("It's even");
        } else {
            println!("It's odd");
        }
    });
}
