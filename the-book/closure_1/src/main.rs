#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for sh in &self.shirts {
            match sh {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue],
    };

    let u1 = Some(ShirtColor::Red);
    let u2 = None;

    let sh1 = store.giveaway(u1);
    println!(
        "for user1 with preference {:?} the giveaway shirt is {:?}",
        u1, sh1
    );

    let sh2 = store.giveaway(u2);
    println!(
        "for user2 with preference {:?} the giveaway shirt is {:?}",
        u2, sh2
    );
}
