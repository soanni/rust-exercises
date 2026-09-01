use std::fmt::{Debug, Display};

#[derive(Debug)]
struct Monster {
    health: u32,
}

#[derive(Debug)]
struct Wizard {
    health: u32,
}

#[derive(Debug)]
struct Ranger {
    health: u32,
}

impl Ranger {
    fn new(health: u32) -> Self {
        Self { health }
    }
}

impl Wizard {
    fn new(health: u32) -> Self {
        Self { health }
    }
}

impl Monster {
    fn new(health: u32) -> Self {
        Self { health }
    }
}

impl std::fmt::Display for Monster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remaining monster's health is {}", self.health)
    }
}

impl std::fmt::Display for Wizard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remaining wizard's health is {}", self.health)
    }
}

impl std::fmt::Display for Ranger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Remaining ranger's health is {}", self.health)
    }
}

trait FightFromDistance {}
trait FightClose {}
trait Magic {}

impl FightClose for Wizard {}
impl FightClose for Ranger {}
impl FightFromDistance for Ranger {}
impl Magic for Wizard {}

trait Attackable {
    fn take_damage(&mut self, damage: u32);
}

trait Descriptive: Display {
    fn show_details(&self) {
        println!("{self}");
    }
}

impl Attackable for Monster {
    fn take_damage(&mut self, damage: u32) {
        self.health -= damage;
    }
}

impl Descriptive for Monster {}
impl Descriptive for Ranger {}
impl Descriptive for Wizard {}

fn attack_with_bow<T, V>(player: &T, opponent: &mut V, distance: u32)
where
    T: FightFromDistance + Descriptive,
    V: Attackable + Descriptive,
{
    if distance < 10 {
        //opponent.health -= 10;
        opponent.take_damage(10);
        //println!(
        //    "Attack with bow! Opponent health is {}. You are at {opponent:?}",
        //    opponent.health
        //);
        print!("Attack with bow !!! *** ");
        opponent.show_details();
        player.show_details();
    }
}

fn attack_with_sword<T, V>(player: &T, opponent: &mut V)
where
    T: FightClose + Descriptive,
    V: Attackable + Descriptive,
{
    //opponent.health -= 4;
    opponent.take_damage(4);
    //println!(
    //    "Attack with sword! Opponent health is {}. You are at {opponent:?}",
    //    opponent.health
    //);
    print!("Attack with sword !!! *** ");
    opponent.show_details();
    player.show_details();
}

fn fireball<T: Magic + Descriptive, V: Attackable + Descriptive>(
    player: &T,
    opponent: &mut V,
    distance: u32,
) {
    if distance < 15 {
        //opponent.health -= 18;
        opponent.take_damage(18);
        //println!(
        //    "Massive fireball! Opponent health is {}. You are at {opponent:?}",
        //    opponent.health
        //);
        print!("Massive fireball !!! *** ");
        opponent.show_details();
        player.show_details();
    }
}

fn main() {
    let ranger = Ranger::new(100);
    let wizard = Wizard::new(90);
    let mut monster = Monster::new(120);
    attack_with_bow(&ranger, &mut monster, 9);
    attack_with_sword(&wizard, &mut monster);
    attack_with_sword(&ranger, &mut monster);
    fireball(&wizard, &mut monster, 12);
}
