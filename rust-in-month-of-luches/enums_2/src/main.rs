enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_skystate(time: u8) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun("i can see the sun".to_string()),
        _ => ThingsInTheSky::Stars("i can see the stars".to_string()),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(desc) => println!("{desc}"),
        ThingsInTheSky::Stars(desc) => println!("{desc}"),
    }
}

fn main() {
    let state = create_skystate(9);
    check_skystate(&state);
}
