use std::rc::Rc;

fn takes_string(s: Rc<String>) {}

fn main() {
    let ss = Rc::new(String::from("stringstring"));

    takes_string(Rc::clone(&ss));
    println!("number of strong counts is: {}", Rc::strong_count(&ss));

    takes_string(Rc::clone(&ss));

    println!("number of strong counts is: {}", Rc::strong_count(&ss));
}
