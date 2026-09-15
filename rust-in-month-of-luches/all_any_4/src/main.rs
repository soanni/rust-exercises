fn is_char_inside(v: &Vec<char>, ch: char) {
    println!("char {} is inside: {}", ch, v.iter().any(|x| *x == ch));
}

fn main() {
    let v = ('a'..'働').collect::<Vec<char>>();
    println!("len of vector v is {}", v.len());
    is_char_inside(&v, 'a');
    is_char_inside(&v, 'Z');
    is_char_inside(&v, '鑿');
    let vv = ('A'..'z').collect::<Vec<char>>();
    println!("len of vector vv is {}", vv.len());
    println!(
        "Are all characters alphanumeric: {}",
        vv.iter().all(|x| x.is_alphanumeric())
    );
}
