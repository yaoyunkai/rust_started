/*
calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权

*/

fn func1() {
    let s1 = String::from("hello， 你好");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn func2() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s after changed is {s}");
}

fn change(s: &mut String) {
    String::push_str(s, ", world");
}

fn func3() {
    // let mut s = String::from("hello");
    //
    // let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{r1}, {r2}");
}

fn func4() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1: {r1}");
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2: {r2}");
}

fn func5() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{r1}, {r2}");

    let r3 = &mut s; // 大问题

    println!("{r3}");
}

/*
fn dangling_pointer() {
    let r1 = dangle();
}

fn dangle() -> &String {
    // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
*/

pub fn foo() {
    hello_world::print("引用的用法", func1);
    hello_world::print("可变引用的用法", func2);
    hello_world::print("使用大括号分离作用域", func4);
    hello_world::print("不变引用之后再使用可变引用", func5);
}
