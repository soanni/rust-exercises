/// my_closure implement trait Fn which takes the variable from its environment by reference
fn main() {
    let s = String::from("smth-smth-smth");
    let my_closure = || println!("{s}");

    my_closure();
    my_closure();
}
