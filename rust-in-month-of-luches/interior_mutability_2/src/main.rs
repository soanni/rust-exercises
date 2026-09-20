use std::cell::RefCell;

struct User {
    username: String,
    active: RefCell<bool>,
    year_registered: u32,
}

fn main() {
    let soanni = User {
        username: "soanni".to_string(),
        active: RefCell::new(true),
        year_registered: 2003,
    };
    println!("{:?}", soanni.active);
    *soanni.active.borrow_mut() = false;
    println!("{:?}", soanni.active);
}
