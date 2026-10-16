fn main() {
    let x = |x| x;

    println!("{}", x(String::from("hello")));
    println!("{}", x(5));
}
