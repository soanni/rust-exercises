fn main() {
    let v = vec![1, 2, 3];

    for i in 0..=10 {
        if let Some(n) = v.get(i) {
            println!("{n}");
        }
        let Some(n) = v.get(i) else {
            continue;
        };
        println!("{n}");
    }
}
