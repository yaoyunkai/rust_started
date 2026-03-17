/*
字符串的用法

字符串 slice str，它通常以被借用的形式出现，&str。

String 或 string slice &str

Rust 的字符串不支持索引。

*/
fn new_string() {
    let mut s = String::new();

    println!("{}", s);
    let data = "initial contents";

    s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    // let s = "initial contents".to_string();
    println!("{s}");

    let s = String::from("initial contents");
    println!("从 from 中创建字符, {s}");
}

fn update_string() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{s}");

    s.push('a');
    s.push(' ');
    s.push('b');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 = [{s3}]");
    println!("s2 = [{s2}]");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("new s = {s}");
    println!("{s1}-{s2}-{s3}"); // 不转移所有权
}

pub fn foo() {
    hello_world::print("创建字符串", new_string);
    hello_world::print("更新字符串", update_string);
}
