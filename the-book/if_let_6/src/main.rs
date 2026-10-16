#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    California,
    Miami,
    Colorado,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Miami => year >= 1820,
            UsState::Colorado => year >= 1821,
            _ => year > 1822,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(c: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = c {
        state
    } else {
        return None;
    };

    if state.existed_in(1820) {
        Some(format!("{state:?} is pretty old for America."))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
    //   if let Coin::Quarter(state) = c {
    //        if state.existed_in(1820) {
    //            Some(format!("{state:?} is pretty old for America."))
    //        } else {
    //            Some(format!("{state:?} is relatively new."))
    //        }
    //    } else {
    //        None
    //    }
}

fn main() {
    let mut counter = 0;

    let coins = [
        Coin::Penny,
        Coin::Quarter(UsState::Miami),
        Coin::Quarter(UsState::Alabama),
        Coin::Dime,
        Coin::Quarter(UsState::California),
        Coin::Penny,
    ];

    for c in coins {
        //if let Coin::Quarter(state) = c {
        //    println!("That's a quarter from {state:?}");
        //} else {
        //    counter += 1;
        //}

        match c {
            Coin::Quarter(state) => {
                println!("that's a quarter from {state:?}");
            }
            _ => {
                counter += 1;
            }
        }
    }

    println!("there are {counter} non-quarters in the list");
}
