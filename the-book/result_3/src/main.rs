use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("error creating the file: {err:?}");
            })
        } else {
            panic!("error opening the file: {error:?}");
        }
    });
}
