// those below are made at compile type before program runs
// also can't use heap as the program needs to perform a memory allocation first
const NUMBER_OF_MONTHS: u8 = 12;
static SEASONS: [&str; 4] = ["Summer", "Fall", "Winter", "Spring"];

fn main() {
    println!("number of months is {NUMBER_OF_MONTHS}");
    println!("seasons are {SEASONS:?}");
}
