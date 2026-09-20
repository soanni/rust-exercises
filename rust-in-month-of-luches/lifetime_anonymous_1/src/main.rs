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

fn main() {
    println!("Hello, world!");
}
