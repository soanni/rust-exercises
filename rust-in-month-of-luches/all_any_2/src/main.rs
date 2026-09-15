fn main() {
    let mut v = vec![6; 1000];
    v.push(5);
    //    println!("{}", v.iter().rev().any(|&x| x == 5));
    let mut i = v.iter().rev();
    assert_eq!(i.next(), Some(&5));
    assert_eq!(i.next(), Some(&6));
}
