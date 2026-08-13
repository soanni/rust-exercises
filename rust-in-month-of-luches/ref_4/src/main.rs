// TODO: PUT on the site

fn main() {
    let mut my_num = 8;
    let my_ref = &mut my_num;
    *my_ref += 10;
    let my_immut_ref = &my_num;
    println!("my_num = {}", my_immut_ref);
}
