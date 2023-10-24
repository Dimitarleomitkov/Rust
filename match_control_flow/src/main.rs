#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(num) => Some(num + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) =>{
            println!("State quarter from {:?}!", state);
            25   
        }
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::Alaska);
    let coin5 = Coin::Quarter(UsState::Alabama);

    println!("{}", value_in_cents(coin1));
    println!("{}", value_in_cents(coin2));
    println!("{}", value_in_cents(coin3));
    println!("{}", value_in_cents(coin4));
    println!("{}", value_in_cents(coin5));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
}
