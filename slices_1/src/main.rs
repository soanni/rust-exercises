fn main() {
    let w = String::from("abc defgh ijklm");
    println!("{}", first_word(&w));
}

fn first_word(s: &String) -> usize {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
