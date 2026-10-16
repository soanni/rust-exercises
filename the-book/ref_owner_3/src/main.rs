fn main() {
    let mut s = String::from("some string");
    let r1 = &mut s;
    //let r2 = &s;

    println!("{r1}");
}
