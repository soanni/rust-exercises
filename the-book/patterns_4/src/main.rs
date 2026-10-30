fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("you are here ({x}, {y})");
}

fn main() {
    let point = (1, 2);
    print_coordinates(&point);
}
