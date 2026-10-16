fn main() {
    let v = vec![1, 2, 3];
    let mut v_i = v.iter();

    assert_eq!(v_i.next(), Some(&1));
    assert_eq!(v_i.next(), Some(&2));
    assert_eq!(v_i.next(), Some(&3));
    assert_eq!(v_i.next(), None);
}
