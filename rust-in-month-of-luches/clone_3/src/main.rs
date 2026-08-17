fn get_number_of_words(s: &String) {
    println!(
        "There are {} words in the string",
        s.split_whitespace().count()
    );
}

fn main() {
    let mut s = String::new();
    for _ in 0..50 {
        s.push_str("Some more new words");
        get_number_of_words(&s);
    }
}
