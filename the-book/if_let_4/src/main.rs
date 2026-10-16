#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    California,
    Miami,
    Colorado,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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
            Coin::Quarter(state) => println!("that's a quarter from {state:?}"),
            _ => {
                counter += 1;
            }
        }
    }

    println!("there are {counter} non-quarters in the list");
}
