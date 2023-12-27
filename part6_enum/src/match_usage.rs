/*

模式匹配

Rust 中的匹配是 穷尽的（exhaustive）：必须穷举到最后的可能性来使代码有效。


*/


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


pub fn run_match() {
    let num = value_in_cents(Coin::Nickel);
    println!("the num is {num}");

    let num = value_in_cents(Coin::Penny);
    println!("the num is {num}");

    let num = value_in_cents(Coin::Dime);
    println!("the num is {num}");

    let num = value_in_cents(Coin::Quarter);
    println!("the num is {num}");

    let num = value_in_cents1(Coin1::Quarter(UsState::Alabama));
    println!("the num is {num}");


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("the six  is {:?}", six);
    println!("the none is {:?}", none);


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    let dice_roll = 4;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}


#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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
        // 相当于实例化 Enum的语法
        Coin1::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}


fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(num_spaces: u8) {}

fn reroll() {}