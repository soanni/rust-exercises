fn main() {
    let str = String::from("hello");
    let l = calc_len(&str);
    println!("the len of string {str} is {l}");
}

fn calc_len(s: &String) -> usize {
    s.len()
}
