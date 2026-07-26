struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let mut u = User {
        active: true,
        sign_in_count: 1,
        username: String::from("soanni"),
        email: String::from("soanni1986@gmail.com"),
    };
    u.email = String::from("legandr.86@gmail.com");
}
