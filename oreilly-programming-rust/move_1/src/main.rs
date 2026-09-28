fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    //let t = s;
    //let u = s;
    let t = s.clone();
    let u = s.clone();

    let mut ss = "string_1".to_string();
    let tt = ss;
    // println!("{ss}");
    ss = "string_2".to_string();
    println!("{ss}");
}
