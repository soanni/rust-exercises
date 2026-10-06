/// Rust can either unwind the stack when a panic happens or abort the process. Unwinding is the default.
/// A panic is not a crash. It’s not undefined behavior. It’s more like a RuntimeException in Java or a std::logic_error in C‍++. The behavior is well-defined; it just shouldn’t be happening
/// Panic is safe. It doesn’t violate any of Rust’s safety rules; even if you manage to panic in the middle of a standard library method, it will never leave a dangling pointer or a half-initialized value in memory. The idea is that Rust catches the invalid array access, or whatever it is, before anything bad happens. It would be unsafe to proceed, so Rust unwinds the stack. But the rest of the process can continue running.
/// Panic is per thread. One thread can be panicking while other threads are going on about their normal business.
fn main() {
    println!("pirate's share is {}", pirates_share(999, 0));
}

fn pirates_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
