use std::borrow::Cow::{self, Borrowed, Owned};

struct User {
    name: Cow<'static, str>,
}

fn main() {
    let name1 = "user1";
    let name2 = "user2".to_string();

    let u1 = User { name: name1.into() };
    let u2 = User { name: name2.into() };

    for u in [u1, u2] {
        match u {
            User { name: Borrowed(n) } => println!("{n} borrowed"),
            User { name: Owned(n) } => println!("{n} owned"),
        }
    }
}
