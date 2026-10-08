/// Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.
fn main() {
    let v = vec![1, 2, 3];
    println!("before defining the closure - {v:?}");
    let only_borrows = || println!("{v:?}");
    println!("before calling the closure - {v:?}");
    only_borrows();
    println!("after calling the closure - {v:?}");
}
