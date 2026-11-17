enum MapDirection {
    North,
    NorthWest,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

fn main() {
    println!("Hello, world!");

    give_direction(&MapDirection::NorthWest);
}

fn give_direction(d: &MapDirection) {
    use MapDirection::*;

    match d {
        North => println!("you are heading North"),
        NorthWest => println!("you are heading NorthWest"),
        SouthWest => println!("you are heading SouthWest"),
        South => println!("you are heading South"),
        SouthEast => println!("you are heading SouthEast"),
        East => println!("you are heading East"),
        NorthEast => println!("you are heading NorthEast"),
    }
}
