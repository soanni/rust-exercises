fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 255);
    // destructuring via a pattern matching
    let (x, y, z) = tup;
    println!("The value of z is {z}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let two_hundred_fifty_five = tup.2;
    println!("tup.0 = {five_hundred}, tup.1 = {six_point_four}, tup.2 = {two_hundred_fifty_five}");
    let unit = ();
}
