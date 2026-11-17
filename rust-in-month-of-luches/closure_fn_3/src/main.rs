/// closure takes ownership on the vector so it doesnt exit after the first call. my_closure
/// implements FnOnce
fn main() {
    let v = vec![1, 2, 3, 4];
    //let my_closure = || v.iter().for_each(|x| println!("{x}"));
    let my_closure = || v.into_iter().for_each(|x| println!("{x}"));
    my_closure();
    //my_closure();
}
