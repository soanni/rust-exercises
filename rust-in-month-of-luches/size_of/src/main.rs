fn main() {
    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_u8 = std::mem::size_of::<u8>();
    let size_of_f64 = std::mem::size_of::<f64>();

    let size_of_str_1 = std::mem::size_of_val("자우림");
    let size_of_str_2 = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("size_of_string = {}", size_of_string);
    println!("size_of_i8 = {}", size_of_i8);
    println!("size_of_u8 = {}", size_of_u8);
    println!("size_of_f64 = {}", size_of_f64);
    println!("size_of_str_1 = {}", size_of_str_1);
    println!("size_of_str_2 = {}", size_of_str_2);
}
