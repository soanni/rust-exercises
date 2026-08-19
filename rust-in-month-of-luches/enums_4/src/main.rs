enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn check_the_mood(mood: &Mood) -> u8 {
    use Mood::*;
    let level = match mood {
        Happy => 10,
        Sleepy => 7,
        NotBad => 5,
        Angry => 1,
    };
    level
}

fn main() {
    let mood = Mood::Sleepy;
    let level = check_the_mood(&mood);
    println!("Out of 10 my level of mood is {level}");
}
