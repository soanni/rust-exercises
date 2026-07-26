fn main() {
    let num = 6;

    if num % 4 == 0 {
        println!("the number is divisible by 4");
    } else if num % 3 == 0 {
        println!("the number is divisible by 3");
    } else if num % 2 == 0 {
        println!("the number is divisible by 2");
    } else {
        println!("the number is not divisible by 4, 3, 2");
    }
}
