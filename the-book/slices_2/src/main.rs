fn main() {
    let w = String::from("abc defgh ijklmn");
    println!("{}", first_word(&w));
}

fn first_word(s: &String) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
