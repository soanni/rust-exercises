use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f_result = File::open("hello.txt");

    let f = match f_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(err) => panic!("can't create a file due to {err:?}"),
            },
            _ => panic!("some other error: {error:?}"),
        },
    };
}
