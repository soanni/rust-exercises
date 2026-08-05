use std::fs::File;

fn main() {
    let f_result = File::open("hello.txt");

    let f = match f_result {
        Ok(file) => file,
        Err(error) => panic!("the error opening the file is: {error:?}"),
    };
}
