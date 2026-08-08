use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

//fn get_last_char_on_the_first_line(text: &str) -> Option<char> {
//    text.lines().next()?.chars().last()
//}
