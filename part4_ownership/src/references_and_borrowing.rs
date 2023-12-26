/*

引用和借用


在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
引用必须总是有效的。

*/


pub fn test_reference() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // 当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。
    s.len()
}


// fn test_change_something() {
//     let s = String::from("hello");
// 
//     change(&s);
// }
// 
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


pub fn test_mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s);

    println!("after change, the s is {s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn test_reference_multi_times() {
    /*
    如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。
     */

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    let mut s2 = String::from("hello");
    {
        let r3 = &mut s2;
        println!("{r3}");
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r4 = &mut s;
    println!(" {r4}");
}


pub fn test_reference_multi_times2() {
    // 不能在拥有不可变引用的同时拥有可变引用。

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println!("{}, {}, and {}", r1, r2, r3);
}

pub fn test_reference_multi_times3() {
    /*
    注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    
     */

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

pub fn test_dangling_reference() {
    let reference_to_nothing = dangle();
    let s = no_dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。


fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

