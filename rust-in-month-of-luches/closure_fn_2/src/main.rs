/// my_closure implement trait FnMut which takes the mut ref from its environment
fn main() {
    let mut s = String::from("smth-smth-smth");
    let mut my_closure = || {
        s.push_str(" new");
        println!("{s}")
    };

    my_closure();
    my_closure();
}
