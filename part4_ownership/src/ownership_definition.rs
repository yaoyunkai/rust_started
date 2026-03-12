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
    // let s = String::from("hello");

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{s}"); // 将打印 `hello, world!`

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}, world!"); // compile error

    // 当你给一个已有的变量赋一个全新的值时，Rust 将会立即调用 drop 并释放原始值的内存。
    let mut s = String::from("hello");
    // print!("{s}");
    s = String::from("ahoy");
    println!("{s}, world!");

}

fn string_clone_usage() {
    // 如果我们 确实 需要深度复制 String 中堆上的数据，
    // 而不仅仅是栈上的数据，可以使用一个叫做 clone 的常用方法。
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

pub fn foo() {
    hello_world::print("变量的作用域", variables_scope);
    hello_world::print("字符串的使用", string_usage);
    hello_world::print("字符串clone方法的使用", string_clone_usage);
}
