/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}

unwrap
expect

TODO ? 运算符快捷方式
被 ? 作用的错误值会经过 from 函数。
这个函数定义在标准库的 From trait 中，用于把一种类型的值转换成另一种类型。
当 ? 运算符调用 from 函数时，接收到的错误类型会被转换成当前函数返回类型里定义的错误类型。

*/
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn open_file() {
    let fp = File::open("hello.txt");

    let greeting_file = match fp {
        Ok(_f) => _f, // 这里的结果返回给了 变量 greeting_file
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    println!("greeting file is {:?}", greeting_file);
}

fn open_file2() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    println!("greeting file is {:?}", greeting_file);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn foo() {
    // hello_world::print("打开一个文件", open_file);
    hello_world::print("打开一个文件", open_file2);
}
