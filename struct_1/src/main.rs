struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let u = User {
        active: true,
        sign_in_count: 1,
        username: String::from("soanni"),
        email: String::from("soanni1986@gmail.com"),
    };
}
