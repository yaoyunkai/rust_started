/*

slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。

字符串字面值就是 slice

*/

pub fn run_slice_1() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 的值为 5
    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


pub fn run_slice_2() {

    // s2 它是一个指向二进制程序特定位置的 slice。
    // 这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。
    let s2 = "Hello, world!";
    println!("the s2 is {s2}");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello world");

    let word = first_word2(&s);

    s.clear(); // 错误！

    println!("the first word is: {}", word);
}


fn first_word2(s: &String) -> &str {
    // 返回了 s的引用

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn run_slice_3() {
    // Deref
    
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word3(my_string_literal);
}

fn first_word3(s: &str) -> &str {
    // 返回了 s的引用

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
