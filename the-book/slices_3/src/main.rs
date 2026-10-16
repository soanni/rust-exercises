fn main() {
    let mut w = String::from("abc defgh ijklmn");
    let fw = first_word(&w);
    w.clear();
    println!("{}", fw);
}

fn first_word(s: &String) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
