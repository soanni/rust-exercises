fn return_country() -> &String {
    let c = String::from("Russia");
    let r = &c;
    r
}

fn main() {
    let rc = return_country();
    //    println!("{}", rc);
}
