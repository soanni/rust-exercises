fn see_if_its_even(n: i32) -> Result<(), ()> {
    if n % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    if see_if_its_even(5).is_ok() {
        println!("its Ok");
    } else {
        println!("its Err");
    }
}
