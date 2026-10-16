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

    let u1 = build_user(String::from("elmira"), String::from("elmira@mail.ru"));
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        username: username,
        email: email,
    }
}
