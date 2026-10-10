/// Iterator adapters are methods defined on the Iterator trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.

/// Code shows an example of calling the iterator adapter method map, which takes a closure to call on each item as the items are iterated through. The map method returns a new iterator that produces the modified items. The closure here creates a new iterator in which each item from the vector will be incremented by 1.
/// The code in doesn’t do anything; the closure we’ve specified never gets called. The warning reminds us why: Iterator adapters are lazy, and we need to consume the iterator here.
fn main() {
    let v = vec![1, 2, 3];

    v.iter().map(|x| x + 1);
}
