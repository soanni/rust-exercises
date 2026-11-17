use std::borrow::Cow::{self, Borrowed, Owned};

#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}

#[derive(Debug)]
struct ErrorInfo {
    msg: String,
    err_type: LocalError,
}

fn generate_message(msg: &'static str, err_info: Option<ErrorInfo>) -> Cow<'static, str> {
    match err_info {
        Some(e) => format!("{msg}: {e:?}").into(),
        None => msg.into(),
    }
}

fn main() {
    let err1 = generate_message("Everything is fine", None);
    let err2 = generate_message(
        "Got an error",
        Some(ErrorInfo {
            msg: "Value is too big".to_string(),
            err_type: LocalError::TooBig,
        }),
    );

    for e in [err1, err2] {
        match e {
            Borrowed(s) => println!("Value {s:?} is borrowed"),
            Owned(s) => println!("Value {s:?} is owned"),
        }
    }
}
