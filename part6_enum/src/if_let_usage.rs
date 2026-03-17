/*
if let 和 let else

可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

*/

fn if_let_usage() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(v) = config_max {
        println!("The maximum is configured to be {v}");
    }

    let num = 32u8;

    if let 3 = num {
        println!("num value is 3");
    } else {
        println!("num value is not 3");
    }
}

fn process_user_input(input: Option<String>) {
    // 使用 let else 提取 Some 中的值
    // 如果 input 是 None，则执行 else 块并提前 return
    let Some(text) = input else {
        println!("错误：未提供输入，提前退出函数。");
        return; // else 块必须发散（diverge），即必须中断控制流
    };

    // 此时，text 已经被提取出来，并且可以在当前作用域的剩余部分直接使用！
    // 不需要像 if let 那样把后续逻辑写在括号里。
    println!("成功接收到用户输入: {}", text);
    println!("输入的长度为: {}", text.len());
}

/*
在 Rust 开发中，直接使用 .unwrap() 通常被视为一种“坏味道”（Code Smell），因为它有潜在的 panic 风险。
虽然在这个特定的上下文中，你通过前面的 if 保证了它绝对安全，
但编译器并不知道这一点，它依然把 input 当作 Option<String>。
如果以后有人重构代码，不小心把 if 判断删了，.unwrap() 就会导致程序崩溃。

*/
fn process_user_input2(input: Option<String>) {
    // 1. 先判断是否为 None
    if input.is_none() {
        println!("错误：未提供输入");
        return;
    }

    // 2. 此时必须使用 .unwrap() 才能把值取出来
    let text = input.unwrap();

    println!("输入内容: {}", text);
}

fn let_else_usage() {
    process_user_input(Some(String::from("Hello World!")));
    process_user_input(None);
}

pub fn foo() {
    hello_world::print("if let   的用法", if_let_usage);
    hello_world::print("let else 的用法", let_else_usage);
}
