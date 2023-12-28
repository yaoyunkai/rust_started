/*

Result

unwrap 
expect

Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
如果值是 Err，Err 将作为整个函数的返回值，


? 可以作用于 Result和Option

*/




use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

#[allow(unused)]
pub fn run_with_result() {
    let greeting_file_result = File::open("docs/hello.txt");

    #[allow(unused)] let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


#[allow(dead_code)]
pub fn run_with_result_1() {
    let file_path = "docs/hello.txt";

    let greeting_file_result = File::open(file_path);

    #[allow(unused)] let mut greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("Oh.. get ok");
            file
        }
        Err(error) => {
            println!("Oh.. get error");
            println!("error is {:?}", error);

            match error.kind() {
                ErrorKind::NotFound => {
                    println!("file is not found, will create");
                    match File::create(file_path) {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    }
                }
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            } // error.kind
        } // Err
    };

    let mut data = String::new();
    greeting_file.read_to_string(&mut data).expect("error");
    println!("final data is {data}");
}


#[allow(unused)]
pub fn run_result_with_unwrap() {
    let greeting_file = File::open("docs/no.txt").unwrap();
    let greeting_file = File::open("hello.txt").expect("no file no.txt in docs");
}


#[allow(unused)]
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

#[allow(unused)]
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(unused)]
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}


#[allow(unused)]
fn read_username_from_file4() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}