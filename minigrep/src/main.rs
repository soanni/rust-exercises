use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    //dbg!(args);
    let query = args.get(1).unwrap();
    let file_path = args.get(2).unwrap();

    println!("searching for {query}");
    println!("in file {file_path}");

    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");
    println!("with contents:\n{contents}");
}
