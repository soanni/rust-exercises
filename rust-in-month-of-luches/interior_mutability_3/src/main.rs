use std::cell::RefCell;

fn main() {
    let v = RefCell::new(true);
    std::thread::spawn(|| {
        *v.borrow_mut() = false;
    });
}
