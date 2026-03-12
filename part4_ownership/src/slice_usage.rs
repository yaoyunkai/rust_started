/*
切片 slice

编写一个函数，接收一个由空格分隔单词的字符串，并返回它在该字符串中找到的第一个单词。

*/

fn first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn func1() {
    let mut s = String::from("hello world");

    let word = first_word_v1(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}

fn slice_of_string() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn func2() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // 错误！
    println!("the first word is: {word}");

    let s2 = "asdfwqe ddddddddddddd";
    let word2 = first_word(s2);
    println!("the first word2 is {word2}");
}

pub fn foo() {
    hello_world::print("字符串slice的使用", func2);
}
