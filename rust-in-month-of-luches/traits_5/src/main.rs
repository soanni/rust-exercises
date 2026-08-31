use std::fmt::Debug;

trait FightFromDistance: Debug {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!(
                "Bow attack! Opponent has {} health left. You are at {:?}",
                opponent.health, self
            );
        }
    }

    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 8 {
            opponent.health -= 4;
            println!(
                "Rock attack. Opponent has {} health left. You are at {:?}",
                opponent.health, self
            );
        }
    }
}

trait FightClose: Debug {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "Sword attack. Opponent has {} health left. You are at {:?}",
            opponent.health, self
        );
    }
}

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait DisplayHealth {
    fn get_health(&self) -> i32;
}

impl FightClose for Wizard {}
impl FightClose for Ranger {}
impl FightFromDistance for Ranger {}

fn main() {
    let ranger = Ranger { health: 80 };
    let wizard = Wizard { health: 60 };
    let mut monster = Monster { health: 100 };

    wizard.attack_with_sword(&mut monster);
    ranger.attack_with_bow(&mut monster, 8);
    ranger.attack_with_rock(&mut monster, 7);
}
