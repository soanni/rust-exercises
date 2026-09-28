/// A slice, written [T] without specifying the length, is a region of an array or vector.
/// Since a slice can be any length, slices can’t be stored directly in variables or passed as function arguments.
/// They are always passed by reference.
/// This being the case, it’s extremely common to refer to references of type &[T] as “slices,” using the shorter name for the more common concept.
/// We’ll do the same later on, but in this section, we’ll call values of type &[T] “slice references” to avoid ambiguity.

fn main() {
    let v = vec![0.0, 1., 2.2, 2.9, 9.8];
    let arr = [0.0, 1., 2.2, 2.9, 9.8];

    let v_s: &[f64] = &v;
    let arr_s: &[f64] = &arr;

    dbg!(v_s);
    dbg!(arr_s);

    print_nums(&v);
    println!();
    print_nums(&arr);

    print_nums(&v[1..2]);
    print_nums(&arr[2..]);
    print_nums(&v_s[2..3]);
    //print_nums(v_s[2..3]);
}

/// Where possible, functions should accept slice references rather than vector or array references, for generality.
fn print_nums(v: &[f64]) {
    for el in v {
        println!("{el}");
    }
}
