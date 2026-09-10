use std::fmt;

#[derive(Debug, Clone)]
struct File(String);

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_bytes = format!("{:?}", self.0.as_bytes());
        write!(f, "{as_bytes}")
    }
}

fn main() {
    let my_file = File(String::from("some string"));
    println!("{my_file:?}");
    println!("{my_file}");
}
