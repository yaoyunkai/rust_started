/*
两类数据类型子集：标量（scalar）和复合（compound）

Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。

Rust 中的布尔类型使用 bool 表示。

Rust 的 char 类型的大小为四个字节 (four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value）


复合类型

tuple and array
不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。
这种值以及对应的类型都写作 ()，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。


*/

use std::io;

pub fn data_compute() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;
}

pub fn run_char() {
    let a: char = 'a';
    let chinese_ni: char = '\u{4f60}';

    let num: u32 = u32::from(chinese_ni);

    println!("the word is {chinese_ni}");
    decimal_to_hex(num);
}


fn decimal_to_hex(decimal_number: u32) {
    // 使用format!宏将数字转换为十六进制字符串
    let hex_string = format!("{:X}", decimal_number);

    // 返回结果
    println!("hex value is 0x{hex_string}");
}

pub fn run_tuple() {
    let tup = (500, 6.4, 1);

    // 解构（destructuring）
    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

pub fn run_array() {
    let months = [
        "January", "February", "March",
        "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];

    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    let first = a[0];
    let second = a[1];

    println!("the first is {first}");
    println!("the first is {second}");
}

pub fn test_out_of_index() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
