use std::env;
use std::str::FromStr;

fn main() {
    //    println!("gcd 125 and 100 is {}", gcd(125, 100));
    let mut numbers = Vec::new();

    for n in env::args().skip(1) {
        numbers.push(u64::from_str(&n).expect("can't parse the u64 from str"));
    }

    if numbers.len() == 0 {
        println!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for n in &numbers[1..] {
        d = gcd(d, *n);
    }

    println!("the gcd of numbers {numbers:?} is {d}");
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    while (m != 0) {
        if (m < n) {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(3 * 5 * 54321, 4 * 7 * 54321), 54321);
    assert_eq!(gcd(0, 0), 0);
}
