struct OneOne;

impl Iterator for OneOne {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

fn main() {
    let fiveones = OneOne.into_iter().take(5).collect::<Vec<i32>>();
    println!("{fiveones:?}");
}
