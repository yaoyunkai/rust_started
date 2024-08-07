/*

闭包

可以保存在一个变量中或作为参数传递给其他函数的匿名函数。

unwrap_or_else:

获取一个没有参数、返回值类型为 T 的闭包作为参数

这是一个本身不获取参数的闭包（如果闭包有参数，它们会出现在两道竖杠之间）

闭包类型推断和注解

 */

use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
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

        for color in &self.shirts {
            match color {
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

pub fn runner1() {
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

pub fn runner2() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    println!("Hha, {}", expensive_closure(34));
}

pub fn runner3() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    } // 函数定义
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 闭包定义
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    let result1 = add_one_v3(4);
    let result2 = add_one_v4(6);

    // let result3 = add_one_v4(4.5);

    println!("result 2 is {result2}");
}

/*
捕获引用或者移动所有权

直接对应到函数获取参数的三种方式：不可变借用，可变借用和获取所有权。

move 关键字: 强制闭包获取它用到的环境中值的所有权

*/
pub fn runner4() {
    // 不可变借用
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // 可变借用
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // 使用 move 强制闭包获取它用到的环境中值的所有权
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    // println!("after defining closure: {:?}", list);
}

/*
FnOnce   : 一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次。
FnMut    : 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值。
Fn       : 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包


 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn runner5() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

pub fn runner6() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
