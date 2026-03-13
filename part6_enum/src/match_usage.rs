enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 0b0001,
        Coin::Nickel => 0b0010,
        Coin::Dime => 0b0011,
        Coin::Quarter => 0b0100,
    }
}

fn run_with_match() {
    let c1 = Coin::Penny;
    let ret1 = value_in_cents(c1);
    println!("ret1 value: {ret1}");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents1(coin: Coin1) -> u8 {
    match coin {
        Coin1::Penny => 1,
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter(_value) => {
            println!("State quarter from {_value:?}!");
            25
        }
    }
}

pub fn foo() {
    hello_world::print("match的使用方式", run_with_match);
}
