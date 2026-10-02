///The . operator also implicitly dereferences its left operand, if needed
/// The . operator can also implicitly borrow a reference to its left operand, if needed for a method call. For example, Vec’s sort method takes a mutable reference to the vector, so these two calls are equivalent
/// In short, Rust automatically creates references and dereferences them in a few places, for convenience. Everywhere else, the & and * operators are required, for clarity.
/// Like the . operator, Rust’s comparison operators “see through” any number of references
/// Arithmetic operators can see through one level of references.

struct A {
    name: String,
    age: i32,
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);
    assert_eq!(x, 10);

    let mut y = 32;
    let z = &mut y;
    *z += 32;
    assert_eq!(*z, 64);
    assert_eq!(y, 64);

    let a = A {
        name: "hello".to_string(),
        age: 39,
    };

    let ra = &a;

    assert_eq!(ra.name, "hello".to_string());

    let x = 1;
    let y = 2;
    let mut rx = &x;
    let b = true;

    if b {
        rx = &y;
    }

    assert!(*rx == 1 || *rx == 2);

    let p = Point { x: 1, y: 2 };

    let pr = &p;
    let prr = &pr;
    let prrr = &prr;

    println!("x = {}, y = {}", prrr.x, prrr.y);

    let v = 10;
    let w = 10;
    let rv = &v;
    let rw = &w;
    let rrv = &rv;
    let rrw = &rw;

    assert!(rrv <= rrw);
    assert!(rrv == rrw);

    assert!(!std::ptr::eq(rrv, rrw));

    //assert!(rv == rrv);
    assert!(rv == *rrv);

    let rf = &factorial(9);

    // In situations like this, Rust simply creates an anonymous variable to hold the expression’s value and makes the reference point to that. The lifetime of this anonymous variable depends on what you do with the reference
    println!("rf + &1000 = {}", rf + &1000);
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}
