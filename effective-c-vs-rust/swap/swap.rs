fn main() {
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    println!("main: a = {a}, b = {b}");
}

fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
    println!("swap: a = {a}, b = {b}");
}
