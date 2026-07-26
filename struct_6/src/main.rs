struct User {
    username: &str,
    email: &str,
    active: bool,
    sign_in_count: u32,
}

fn main() {
    let u1 = User {
        username: "soanni",
        email: "soanni1986@gmail.com",
        active: true,
        sign_in_count: 1,
    };
}
