enum ThingsInTheSky {
    Sun,
    Stars,
}

fn create_skystate(time: u8) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("i can see the sun"),
        ThingsInTheSky::Stars => println!("i can see the stars"),
    }
}

fn main() {
    let state = create_skystate(9);
    check_skystate(&state);
}
