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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_with_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five plus one is {}", six.unwrap());
    println!(
        "none plus one is {:?}",
        if none.is_none() {
            None
        } else {
            Some(none.unwrap())
        }
    );
}

fn match_with_others() {
    fn add_fancy_hat() {
        println!("增加一个帽子");
    }

    fn remove_fancy_hat() {
        println!("移除一个帽子");
    }

    fn move_player(num: u32) {
        println!("移动！{num}");
    }

    fn reroll() {
        println!("reroll");
    }

    let roll = 9u32;

    match roll {
        3 => {
            add_fancy_hat();
        }
        7 => {
            remove_fancy_hat();
        }
        other => {
            move_player(other);
        }
    }

    let roll = 8u32;
    match roll {
        1 => {
            add_fancy_hat();
        }
        _ => {
            reroll();
            // ();
        }
    }
}

pub fn foo() {
    hello_world::print("match的使用方式", run_with_match);
    hello_world::print("option的模式匹配", match_with_option);
    hello_world::print("通配模式和占位符", match_with_others);
}
