use std::{thread, time::Duration};

fn generate_workout(intensity: u32, random_n: u32) {
    let expensive_closure = |n: u32| -> u32 {
        println!("... doing smth complex ...");
        thread::sleep(Duration::from_secs(3));
        n
    };

    if intensity < 25 {
        println!("do {} pushups", expensive_closure(intensity));
    } else {
        if random_n == 3 {
            println!("Take a break today");
        } else {
            println!("Run for {} minutes", expensive_closure(intensity));
        }
    }
}

fn main() {
    let i = 10;
    let n = 7;
    generate_workout(i, n);
}
