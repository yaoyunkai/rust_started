/*
snake case

函数体由一系列的语句和一个可选的结尾表达式构成。

语句（Statements）是执行一些操作但不返回值的指令。 
表达式（Expressions）计算并产生一个值。


在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。

行尾加上一个分号，把它从表达式变成语句

*/


pub fn test_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let five = five();

    println!("Get value for function, value is {five}");
}

fn five() -> i32 {
    5
}
