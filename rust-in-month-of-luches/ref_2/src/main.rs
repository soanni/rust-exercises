fn main() {
    let mut my_num = 8;
    let my_ref = &mut my_num;
    *my_ref += 10;
    println!("my_num = {}", my_num);

    let second_num = 888;
    let triple_ref = &&&second_num;

    println!(
        "second_num == ***tripple_ref = {}",
        second_num == ***triple_ref
    );
}
