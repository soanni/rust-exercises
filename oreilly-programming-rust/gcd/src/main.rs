fn main() {
    println!("gcd 125 and 100 is {}", gcd(125, 100));
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
