fn main() {
    // same as |x| x % 2 == 0
    let f = |x: i64| -> bool { x % 2 == 0 };
    assert_eq!(f(14), true);
}
