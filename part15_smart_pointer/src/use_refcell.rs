/*
内部可变性

Mock对象测试。

*/


mod utils;

fn demo1() {
    let mut x = 5;

    let y = &mut x;
    *y = 10;

    println!("x = {x}");
}

pub fn foo() {
    hello_world::print("内部可变性", demo1);
}
