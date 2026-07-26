fn main() {
    let some_dangling_ref = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
