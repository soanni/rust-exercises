fn main() {
    let w = String::from("abc defgh ijklmn");
    let fw = first_word(&w);
    let fw = first_word(&w[..]);
    let fw = first_word(&w[0..3]);
    println!("{}", fw);

    let ww = "hello world";
    let fw = first_word(&ww[..]);
    let fw = first_word(&ww[0..5]);
    let fw = first_word(ww);

    println!("{}", fw);
}

fn first_word(s: &str) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
