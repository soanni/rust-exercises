fn main() {
    let fav_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "39".parse();

    if let Some(c) = fav_colour {
        println!("background colour is you fav colour {c}");
    } else if is_tuesday {
        println!("Tuesday is green");
    } else if let Ok(a) = age {
        if a > 40 {
            println!("Background colour is red");
        } else {
            println!("Background color is orange")
        }
    } else {
        println!("background is purple");
    }
}
