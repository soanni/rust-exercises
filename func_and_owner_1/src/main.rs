fn main() {
    let mut s = String::from("hello");
    s = take_ownership(s);
    println!("{s}");

    let x = 5;
    make_copy(x);
    println!("{x}");
}

fn take_ownership(mut str: String) -> String {
    //println!("{str}");
    str.push_str(", world!");
    str
}

fn make_copy(n: i32) {
    n + 1;
}
