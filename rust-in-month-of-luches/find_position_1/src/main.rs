fn main() {
    let v = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
    println!("{:?}", v.iter().find(|x| *x % 3 == 0));
    println!("{:?}", v.iter().position(|x| *x % 3 == 0));
    println!("{:?}", v.iter().find(|x| *x % 11 == 0));
    println!("{:?}", v.iter().position(|x| *x % 11 == 0));
}
