/*
Rust 的 闭包（closures）是可以保存在变量中或作为参数传递给其他函数的匿名函数。
你可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。

闭包可以通过三种方式捕获其环境中的值，它们直接对应到函数获取参数的三种方式：
不可变借用、可变借用和获取所有权。


*/

use chrono::Local;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
    Green,
    Yellow,
    Pink,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_prefer: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_else 接受一个无参闭包作为参数，该闭包返回一个 T 类型的值
        user_prefer.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut color_counts = HashMap::new();

        /*
        在 for color in &self.shirts 这个循环中，因为我们遍历的是 &self.shirts（对数组的借用），
        所以变量 color 的类型是 &ShirtColor

        or_insert(0) 这一整段代码执行完后，返回的类型是 &mut i32
        在 Rust 中，你不能直接对一个“引用（指针）”做数学运算。

        方法调用（.）的优先级高于解引用（*）

        */
        for color in &self.shirts {
            // *(color_counts.entry(*color).or_insert(0)) += 1;
            *color_counts.entry(*color).or_insert(0) += 1;
        }

        println!("color_counts content is {color_counts:#?}");

        /*
        List of (ShirtColor, i32)

        max_by_key |&(_, count)| 等于 &(ShirtColor, i32)， 这一步结束后，会返回一个包含最大键值对的 Option

        map 是Option里面的方法。上一步得到了类似 (ShirtColor::Blue, 3) 的元组，

        */
        color_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(color, _)| color)
            .expect("Inventory.shirts 保证不为空，但计算时发生了意外")
    }
}

fn run_with_inventory() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn how_to_write_closures() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    let start = Local::now();
    println!("start at: {}", start.format("%Y-%m-%d %H:%M:%S"));
    expensive_closure(45);
    let end = Local::now();
    println!("end at: {}", end.format("%Y-%m-%d %H:%M:%S"));

    // fn add_one_v1(x: u32) -> u32 {
    //     x + 1
    // };
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    // 类型推断要与实际调用一起使用
    add_one_v2(34);
    add_one_v3(45_u32);
    add_one_v4(23_u8);

    // let example_closure = |x| x;

    // 尝试对同一闭包使用不同类型则就会得到类型错误。
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

fn run_closures_with_immutable_ref() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn run_closures_with_mutable_ref() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

fn run_closures_with_move() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let caller = move || println!("From thread: {list:?}");
    thread::spawn(caller).join().unwrap();

    // println!("After calling closure: {list:?}");
}

pub fn foo() {
    hello_world::print("闭包的使用方式", run_with_inventory);
    hello_world::print("怎么样使用闭包", how_to_write_closures);
    hello_world::print("闭包和不可变引用", run_closures_with_immutable_ref);
    hello_world::print("闭包和可变引用", run_closures_with_mutable_ref);
    hello_world::print("闭包和move", run_closures_with_move);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_most_stocked() {
        let inventory = Inventory {
            shirts: vec![
                ShirtColor::Red,
                ShirtColor::Blue,
                ShirtColor::Blue,
                ShirtColor::Red,
                ShirtColor::Blue,
            ],
        };
        let most_color = inventory.most_stocked();
        assert_eq!(most_color, ShirtColor::Blue);
    }
}
