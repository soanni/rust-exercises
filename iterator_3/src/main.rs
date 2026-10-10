/// The Iterator trait has a number of different methods with default implementations provided by the standard library; you can find out about these methods by looking in the standard library API documentation for the Iterator trait. Some of these methods call the next method in their definition, which is why you’re required to implement the next method when implementing the Iterator trait.

/// Methods that call next are called consuming adapters because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete.
fn main() {
    let v = vec![1, 2, 3];
    let v_i = v.iter();
    let v_sum = v_i.sum::<i32>();

    assert_eq!(v_sum, 6);
    //assert_eq!(v_i.next(), Some(&1));
}
