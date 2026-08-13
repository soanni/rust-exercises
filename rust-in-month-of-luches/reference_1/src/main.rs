fn main() {
    let my_number = 8;
    let my_ref = &my_number;
    let my_ref_ref = &my_ref;

    println!("{}", my_ref_ref);
}
