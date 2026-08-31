use std::fmt::Debug;

trait MonsterBehavior: Debug {
    fn take_damage(&mut self, damage: i32);
    fn display_self(&self) {
        println!("Monster is {self:?}");
    }
}

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut MonsterBehavior, distance: u32) {
        print!("Bow attack! ");
        if distance < 10 {
            opponent.take_damage(10);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }

    fn attack_with_rock(&self, opponent: &mut MonsterBehavior, distance: u32) {
        print!("Rock attack! ");
        if distance < 8 {
            opponent.take_damage(8);
        } else {
            println!("Too far away!");
        }
        opponent.display_self();
    }
}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut MonsterBehavior) {
        print!("Sword attack! ");
        opponent.take_damage(10);
        opponent.display_self();
    }
}

#[derive(Debug)]
struct Monster {
    health: i32,
}

impl MonsterBehavior for Monster {
    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

//trait DisplayHealth {
//    fn get_health(&self) -> i32;
//}

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
