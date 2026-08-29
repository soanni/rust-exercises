use std::collections::BinaryHeap;

fn main() {
    let things_to_do = [
        (80, "thing 1"),
        (100, "thing 2"),
        (90, "thing 3"),
        (5, "thing 4"),
        (95, "thing 5"),
        (70, "thing 6"),
        (60, "thing 7"),
        (85, "thing 8"),
    ];

    let mut bin_heap = BinaryHeap::new();

    for item in things_to_do {
        bin_heap.push(item);
    }

    println!("{bin_heap:?}");
}
