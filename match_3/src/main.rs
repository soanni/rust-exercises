#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Hawaii,
    NorthCarolina,
    Oregon,
    Miami,
    Colorodo,
    Missisipi,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        //        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("the quarter from state {state:?}");
            25
        }
        _ => {
            println!("not a penny/nickel/quarter");
            0
        }
    }
}
