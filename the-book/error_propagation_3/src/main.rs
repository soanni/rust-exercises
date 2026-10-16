use std::fs::File;
use std::io::{self, Read};

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("username is {username}"),
        Err(error) => panic!("can't read the username due to: {error:?}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
