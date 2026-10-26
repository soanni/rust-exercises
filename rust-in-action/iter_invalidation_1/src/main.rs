fn main() {
    let mut v = vec!['a', 'b', 'c'];

    for letter in v {
        println!("{}", letter);
        v.push(letter);
    }
}
