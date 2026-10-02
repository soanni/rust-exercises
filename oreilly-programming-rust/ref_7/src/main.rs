struct S<'a> {
    n: &'a i32,
}

struct D<'a> {
    s: S<'a>,
}

fn main() {
    let v = vec![99, 33, 44, 78, 12, 3, 25, 1, 56];

    println!("smallest of {v:#?} is {}", smallest(&v));

    // let s;

    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    // assert_eq!(*s, 0);
    //
    let s;

    {
        let x = 10;
        s = S { n: &x };
    }

    assert_eq!(*s.n, 10);
}

/// We’ve omitted lifetimes from that function’s signature in the usual way. When a function takes a single reference as an argument and returns a single reference, Rust assumes that the two must have the same lifetime. Writing this out explicitly would give us:
/// fn smallest<'a>(v: &'a [i32]) -> &'a i32
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];

    for i in &v[1..] {
        if *i < *s {
            s = i;
        }
    }
    s
}
