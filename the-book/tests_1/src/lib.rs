pub fn prints_and_returns_10(v: i32) -> i32 {
    println!("tha value passed is {v}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silly_test_pass() {
        let res = prints_and_returns_10(8);
        assert_eq!(res, 10);
    }

    #[test]
    fn silly_test_fail() {
        let res = prints_and_returns_10(3);
        assert_eq!(res, 8);
    }
}
