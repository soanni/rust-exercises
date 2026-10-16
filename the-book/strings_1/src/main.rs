fn main() {
    let mut s = String::new();
    let data = "dataa";
    let ss = data.to_string();
    let sss = "initial contents".to_string();

    let mut str_1 = "foo".to_string();
    str_1.push_str("bar");
    println!("{str_1}");

    let s2 = "xyz";
    str_1.push_str(s2);
    println!("{str_1}");
    println!("{s2}");
    str_1.push('l');
    println!("{str_1}");

    let ss1 = "tic".to_string();
    let ss2 = "tac".to_string();
    let ss3 = "toe".to_string();
    let ss4 = format!("{ss1}-{ss2}-{ss3}");
    println!("{ss4}");
    let ss5 = ss1 + &ss2 + &ss3;
    println!("{ss5}");
    //    println!("{ss1}");
}
