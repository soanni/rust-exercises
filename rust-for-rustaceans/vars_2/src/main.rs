fn main() {
    let mut x: i32;
    //assert_eq!(x, 42);
    x = 42;
    let y = &x;
    x = 32;
    assert_eq!(*y, 42);
}
