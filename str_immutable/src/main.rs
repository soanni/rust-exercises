fn main() {
    //    let mut s = "andrei";
    //    s = s + " solodov";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}
