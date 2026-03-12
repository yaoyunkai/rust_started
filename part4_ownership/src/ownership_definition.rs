// #![allow(unused)]
// #![allow(dead_code, unused_variables)]

/*
什么是所有权

*/

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

fn ownership_with_function() {
    let s = String::from("hello");
    _takes_ownership(s);
    // println!("s is {s}");

    let x = 5;
    _make_copy(x);
    println!("x value is {x} after copy");
}

fn _takes_ownership(str: String) {
    println!("{str}");
}

fn _make_copy(some_integer: i32) {
    println!("{some_integer}")
}

fn ownership_with_return() {
    let s1 = _gives_ownership();
    let s2 = String::from("hello");
    let s3 = _takes_and_gives_back(s2);

    // println!("s1 = {s1}, s2 = {s2}, s3={s3}");
    println!("s1 = {s1}, s3={s3}");
}

fn _gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // 返回 some_string 并将其移至调用函数
}

fn _takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string 并移出给调用的函数
}

pub fn foo() {
    hello_world::print("变量的作用域", variables_scope);
    hello_world::print("字符串的使用", string_usage);
    hello_world::print("字符串clone方法的使用", string_clone_usage);
    hello_world::print("所有权和函数的展示", ownership_with_function);
}
