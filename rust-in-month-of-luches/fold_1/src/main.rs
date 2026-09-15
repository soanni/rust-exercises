fn main() {
    let v = vec![3; 1000];
    println!("{}", v.iter().rev().fold(4444, |total, next| total + next));
}
