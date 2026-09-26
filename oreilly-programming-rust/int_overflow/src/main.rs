use std::i32;

fn main() {
    println!("{}", midpoint(7, i32::MAX));
    assert_eq!(100_u16.wrapping_mul(100), 10000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    //assert_eq!(500_i16.wrapping_mul(500), );
    //let _ = usize::MAX.strict_mul(2);
    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!(255_u8.overflowing_add(2), (1, true));
}

fn midpoint(lo: i32, hi: i32) -> i32 {
    (lo + hi) / 2
}
