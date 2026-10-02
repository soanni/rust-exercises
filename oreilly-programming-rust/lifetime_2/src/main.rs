/// If there are multiple lifetimes among your parameters, then there’s no natural reason to prefer one over the other for the return value, and Rust makes you spell out what’s going on.
/// BUT If your function is a method on some type and takes its self parameter by reference, then that breaks the tie: Rust assumes that self’s lifetime is the one to give everything in your return value.
/// Rust assumes that whatever you’re borrowing, you’re borrowing from self
struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    // equivalent for
    // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String>
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() - 1 {
            if self.elements[i].contains(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn main() {}
