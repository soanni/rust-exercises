struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} health points left", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points", self.name, self.hit_points)
    }
}

fn main() {
    let mut monster = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };

    println!("{monster}");
    monster.take_damage();
}
