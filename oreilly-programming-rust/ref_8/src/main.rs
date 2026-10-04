/// Throughout its lifetime, a shared reference makes its referent read-only: you may not assign to the referent or move its value elsewhere
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // r is a shared reference
    let r = &v;
    // v is moves so becomes uninitialized
    let aside = v;
    r[0];
}
