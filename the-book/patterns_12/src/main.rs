fn foo(_: i32, y: i32) {
    println!("this fn uses only y = {y}");
}

fn main() {
    foo(3, 4);
}
