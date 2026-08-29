use std::collections::VecDeque;

fn main() {
    let mut v = VecDeque::from(vec![0; 600_000]);
    for _ in 0..600_000 {
        v.pop_front();
    }
}
