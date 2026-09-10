fn main() {
    let v1 = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&'a'));
    assert_eq!(v1_iter.next(), Some(&'b'));
    assert_eq!(v1_iter.next(), Some(&'c'));
    assert_eq!(v1_iter.next(), Some(&'d'));
    assert_eq!(v1_iter.next(), Some(&'e'));
    assert_eq!(v1_iter.next(), Some(&'f'));
    assert_eq!(v1_iter.next(), Some(&'g'));
    assert_eq!(v1_iter.next(), None);
    assert_eq!(v1_iter.next(), None);
}
