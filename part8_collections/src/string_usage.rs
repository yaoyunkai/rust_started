/*
String

字符串 slice str，它通常以被借用的形式出现，&str

push_str

Deref 强制转换（deref coercion）的技术 你可以将其理解为它把 &s2 变成了 &s2[..]。

 */


#[allow(dead_code)]
pub fn run_with_string() {
    let data = "initial contents";
    let s = data.to_string();
    println!("the string s is {:?}", s);

    let s = String::from("initial contents");
    println!("the string s is {:?}", s);

    println!();
    println!();

    println!("string API usage");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    s1.push(' ');
    s1.push('\u{4f60}');
    s1.push(' ');
    s1.push('\u{1D4F1}');
    s1.push(' ');
    s1.push('\u{2618}');
    println!("s1 is {s1}");


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 因为 add 没有获取参数的所有权，所以 s2 在这个操作后仍然是有效的 String。
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 is {s3}");
    // println!("{s1}");
    println!("{s2}");

    // 使用 format 对多个字符串拼接
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 宏 format! 生成的代码使用引用所以不会获取任何参数的所有权。
    let s = format!("{s1}-{s2}-{s3}");

    println!("s is {s}");
    println!("s1 is {s1}");
}

#[allow(dead_code)]
pub fn run_string_index() {
    // let s1 = String::from("hello");

    // 不支持索引
    // let h = s1[0]; 

    let hello = String::from("Здравствуйте");
    println!("the hello is {hello}");

    let num1 = String::from("नमस्ते");
    println!("the num1 is {num1}");
}

#[allow(dead_code)]
pub fn run_string_slice() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    println!("the s is {s}");
}

#[allow(dead_code)]
pub fn run_iter_string() {
    // .chars
    // .bytes
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
