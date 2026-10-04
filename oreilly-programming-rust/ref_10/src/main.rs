/// we may borrow a mutable reference to the vector, and we may borrow a shared reference to its elements, but those two references’ lifetimes must not overlap.
/// Rust’s rules for mutation and sharing:

/// Shared access is read-only access.
/// Values borrowed by shared references are read-only. Across the lifetime of a shared reference, neither its referent, nor anything reachable from that referent, can be changed by anything. There exist no live mutable references to anything in that structure, its owner is held read-only, and so on. It’s really frozen.

/// Mutable access is exclusive access.
/// A value borrowed by a mutable reference is reachable exclusively via that reference. Across the lifetime of a mutable reference, there is no other usable path to its referent or to any value reachable from there. The only references whose lifetimes may overlap with a mutable reference are those you borrow from the mutable reference itself.
fn main() {
    let mut v = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    append_from_slice(&mut v, &head);
    append_from_slice(&mut v, &tail);
    assert_eq!(v, [0.0, 1.0, 0.0, -1.0]);
    // append_from_slice(&mut v, &v);
}

fn append_from_slice(v: &mut Vec<f64>, s: &[f64]) {
    for e in s {
        v.push(*e);
    }
}
