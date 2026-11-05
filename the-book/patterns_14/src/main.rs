fn main() {
    let t = (1, 2, 3, 4, 5);

    match t {
        (n1, _, n3, _, n5) => println!("Some numbers are: {n1}, {n3}, {n5}"),
    }
}
