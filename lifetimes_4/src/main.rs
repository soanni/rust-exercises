fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let str1 = String::from("abc");
    let res;
    {
        let str2 = String::from("xxxxxxxxxxxxxxxxxxx");
        res = longest(&str1, &str2);
        //        println!("The longest string is {}", res);
    }
    println!("The longest string is {}", res);
}
