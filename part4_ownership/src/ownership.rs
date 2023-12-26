/*
ownership

*/


///
/// 变量的作用域
///
pub fn test_variable_scope() {
    let s = "hello";

    {
        let a = "ni hao";
        print!("this value is a: {a} \n");
    }
    // print!("this value is a: {a}");

    println!("the s is {s}");
}


pub fn test_string() {
    /*
    必须在运行时向内存分配器（memory allocator）请求内存。
    需要一个当我们处理完 String 时将内存返回给分配器的方法。
    
    */

    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`
}


///
/// 变量与数据交互的方式（一）：移动
///
pub fn run_1() {
    let x = 5;
    let y = x;

    println!("the x is {x}");
    println!("the y is {y}");


    // move s1 被 移动 到了 s2
    let s1 = String::from("hello");
    println!("s1 address is {:p}", &s1);

    let s2 = s1;

    println!("s2 address is {:p}", &s2);


    // println!("the x is {s1}");  // 这叫借用 borrowed
    println!("the y is {s2}");
}


///
/// 变量与数据交互的方式（二）：克隆
///
pub fn run_2() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

///
/// 只在栈上的数据：拷贝
///
pub fn run_3() {

    // 没有调用 clone，不过 x 依然有效且没有被移动到 y 中。
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

///
/// 所有权与函数
///
/// 将值传递给函数与给变量赋值的原理相似。向函数传递值可能会移动或者复制
///
pub fn test_function_with_ownership() {
    let s = String::from("hello");

    // 变量在函数里面也可以move ???
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    // println!("value s is {s}");
    println!("value x is {x}");
}


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
    // some_string
} // 这里，some_string 移出作用域并调用 `drop` 方法。

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处


///
/// 返回值与作用域
///
pub fn test_return_value_and_scope() {

    // 发生了move
    let s1 = gives_ownership();
    println!("s1 mem address is {:p}", &s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {             // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域。
    println!("some_string mem address is {:p}", &some_string);

    some_string                              // 返回 some_string 
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}