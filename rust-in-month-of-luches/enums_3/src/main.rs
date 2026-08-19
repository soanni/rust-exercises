enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn check_the_mood(mood: &Mood) -> u8 {
    let level = match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 7,
        Mood::NotBad => 5,
        Mood::Angry => 1,
    };
    level
}

fn main() {
    let mood = Mood::Sleepy;
    let level = check_the_mood(&mood);
    println!("Out of 10 my level of mood is {level}");
}
