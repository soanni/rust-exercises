fn main() {
    assert_eq!(-f32::MIN, f32::MAX);
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    //    assert_eq!(f32::sqrt(5.0) * f32::sqrt(5.0), 5.);
    //    assert_eq!(f64::sqrt(5.0) * f64::sqrt(5.0), 5.);
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}
