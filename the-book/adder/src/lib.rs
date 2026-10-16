pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        //        if value < 1 || value > 100 {
        if value < 1 {
            panic!("the value exceeds the range [1, 100]. the value provided is {value}");
        }
        Guess { value }
    }
}

pub fn greeting(name: &str) -> String {
    //format!("Hello, {name}")
    String::from("Hello")
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
    //    #[should_panic]
    //    fn value_greater_than_100() {
    //        Guess::new(200);
    //    }

    //    #[test]
    //    fn greeting_contains() {
    //        let result = greeting("Andrei");
    //        assert!(
    //            result.contains("Andrei"),
    //            "greeting didn'contain the name, the value was {result}"
    //        );
    //    }

    #[test]
    fn it_adds_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //    #[test]
    //    fn another() {
    //        panic!("make this test fail");
    //    }
    //
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 5,
            height: 11,
        };

        let smaller = Rectangle {
            width: 3,
            height: 8,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 5,
            height: 11,
        };

        let smaller = Rectangle {
            width: 3,
            height: 8,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
