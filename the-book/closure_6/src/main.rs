use std::thread;

/// We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the list. In Listing 13-4, the closure only captured list using an immutable reference because that’s the least amount of access to list needed to print it. In this example, even though the closure body still only needs an immutable reference, we need to specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition. If the main thread performed more operations before calling join on the new thread, the new thread might finish before the rest of the main thread finishes, or the main thread might finish first. If the main thread maintained ownership of list but ended before the new thread and drops list, the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved into the closure given to the new thread so that the reference will be valid.
fn main() {
    let v = vec![1, 2, 3];
    println!("before defining the closure - {v:?}");
    thread::spawn(move || println!("from the thread - {v:?}"))
        .join()
        .unwrap();
    // println!("after calling the closure - {v:?}");
}
