use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("hello.txt")?;
    match get_last_char_on_the_first_line(&contents) {
        Some(ch) => println!("{ch}"),
        None => println!(".. nothing in the file .."),
    }
    Ok(())
}

fn get_last_char_on_the_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
