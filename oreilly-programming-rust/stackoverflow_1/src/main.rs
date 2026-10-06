fn main() {
    f();
}

fn f() {
    let a = [0u8; 1_000_000];
    println!("a.len() = {}", a.len());
    f();
}
