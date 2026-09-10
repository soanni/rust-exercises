struct File(String);

fn main() {
    let my_file = File(String::from("some string"));
    let my_string = String::from("some string");
    println!("my_file == my_string {}", my_file.0 == my_string);
}
