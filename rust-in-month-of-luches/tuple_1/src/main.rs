fn main() {
    let random_tuple = (1, vec![1, 2, 3], [1, 2, 3], 6.6, 'a', "bla-bla-bla");
    println!(
        "Inside the tuple is:
First item: {:?},
Second: {:?}
Third: {:?}
Forth: {:?}
Fifth: {:?}
Sixth: {:?}",
        random_tuple.0,
        random_tuple.1,
        random_tuple.2,
        random_tuple.3,
        random_tuple.4,
        random_tuple.5
    );
}
