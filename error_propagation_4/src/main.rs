use std::fs;
use std::io;

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("username is {username}"),
        Err(error) => panic!("can't read the username due to: {error:?}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
