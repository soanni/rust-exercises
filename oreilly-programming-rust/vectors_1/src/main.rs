fn main() {
    let mut primes = vec![2, 3, 5, 7];

    assert_eq!(primes.iter().product::<i32>(), 210);

    let v: Vec<i32> = (0..5).collect();

    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut v1 = Vec::with_capacity(2);
    assert_eq!(v1.len(), 0);
    assert_eq!(v1.capacity(), 2);

    v1.push(1);
    v1.push(111);

    assert_eq!(v1.len(), 2);
    assert_eq!(v1.capacity(), 2);

    v1.push(333);
    assert_eq!(v1.capacity(), 4);

    primes.insert(4, 11);
    primes.insert(4, 13);

    assert_eq!(primes, [2, 3, 5, 7, 13, 11]);

    primes.remove(2);

    assert_eq!(primes, [2, 3, 7, 13, 11]);

    assert_eq!(primes.pop(), Some(11));
    assert_eq!(primes.pop(), Some(13));
    assert_eq!(primes.pop(), Some(7));

    let langs = std::env::args().skip(1).collect::<Vec<String>>();

    for l in langs {
        println!(
            "{}",
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}

fn new_pixel_buffer(width: usize, height: usize) -> Vec<u8> {
    vec![0_u8; width * height]
}
