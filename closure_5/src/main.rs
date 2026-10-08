/// When borrows_mutably is defined, it captures a mutable reference to list. We don’t use the closure again after the closure is called, so the mutable borrow ends. Between the closure definition and the closure call, an immutable borrow to print isn’t allowed, because no other borrows are allowed when there’s a mutable borrow.
fn main() {
    let mut v = vec![1, 2, 3];
    println!("before defining a closure - {v:?}");
    let mut borrows_mutably = || v.push(4);
    // println!("before calling a closure - {v:?}");
    borrows_mutably();
    println!("after calling a closure - {v:?}");
}
