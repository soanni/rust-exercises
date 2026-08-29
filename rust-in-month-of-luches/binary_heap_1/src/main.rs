use std::collections::BinaryHeap;

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];

    println!("original vector length is {}", many_numbers.len());

    let mut bin_heap = BinaryHeap::new();

    for n in many_numbers {
        bin_heap.push(n);
    }

    println!("binary heap length is {}", bin_heap.len());

    println!("original bin heap is {bin_heap:?}");

    while let Some(n) = bin_heap.pop() {
        println!("popped: {n}. Remaining are: {bin_heap:?}")
    }
}
