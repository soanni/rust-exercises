use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").expect("there is no such damn hello.txt file");
}
