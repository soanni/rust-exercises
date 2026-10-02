static mut STASH: &i32;

fn f(p: &i32) {
    STASH = p;
}

fn main() {
    // static mut STASH: &i32;
}
