enum Seasons {
    Summer,
    Autumn,
    Winter,
    Spring,
}

fn main() {
    use Seasons::*;

    let seas_arr = [Summer, Autumn, Winter, Spring];

    for s in seas_arr {
        println!("{}", s as u32);
    }
}
