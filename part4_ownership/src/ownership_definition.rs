// #![allow(unused)]
// #![allow(dead_code, unused_variables)]

/*
什么是所有权

*/

fn unused_func() {}

fn variables_scope() {
    let num = 30u32;
    {
        let s = "hello world";
    } // s的作用域在此结束。

    // println!("num is {num}, s is {s}");
    println!("num is {num}");
}

///
/// string类型的使用
/// 这个类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本。
///
fn string_usage() {
    let s = String::from("hello");

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{s}"); // 将打印 `hello, world!`
}

pub fn foo() {
    hello_world::print("变量的作用域", variables_scope);
    hello_world::print("字符串的使用", string_usage);
}
