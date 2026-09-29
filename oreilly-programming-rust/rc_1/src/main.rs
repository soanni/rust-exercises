use std::rc::Rc;

fn main() {
    let s = Rc::new("hello".to_string());
    let t = s.clone();
    let u = s.clone();

    // s.push_str(" world!");
}
