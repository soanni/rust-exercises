/// Every static must be initialized.

/// Mutable statics are inherently not thread-safe (after all, any thread can access a static at any time), and even in single-threaded programs, they can fall prey to other sorts of reentrancy problems. For these reasons, you may access a mutable static only within an unsafe block. In this example we’re not concerned with those particular problems, so we’ll just throw in an unsafe block and move on.

static mut STASH: &i32 = &128;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn main() {
    static SMTH: i32 = 1024;

    f(&SMTH);
}
