use std::fs::File;
use std::io::{self, Read};

fn main() {
    match read_username_from_file() {
        Ok(username) => println!("username is {username}"),
        Err(error) => panic!("can't read the username due to: {error:?}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}
