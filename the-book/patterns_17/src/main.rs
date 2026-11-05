fn main() {
    let t = (1, 2, 3, 4, 5);

    match t {
        (first, .., last) => println!("{first} and {last}"),
    }
}
