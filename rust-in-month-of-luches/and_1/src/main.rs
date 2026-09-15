fn main() {
    let try_1 = [Some("Ok"), None, Some("Ok"), Some("Ok")];
    let try_2 = [None, Some("Ok"), Some("Ok"), None];
    let try_3 = [Some("Ok"), None, Some("Ok"), Some("Ok")];
    for i in 0..try_1.len() {
        println!("{:?}", try_1[i].and(try_2[i]).and(try_3[i]));
    }
}
