/// matching ranges of values
fn main() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCI range"),
        'k'..='z' => println!("late ASCI range"),
        _ => println!("smth else"),
    }
}
