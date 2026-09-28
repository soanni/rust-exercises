fn main() {
    let mut v = vec!["one".to_string(), "two".to_string(), "three".to_string()];

    for s in &mut v {
        s.push('!');
        println!("{s}");
    }

    println!("{:?}", v);
}
