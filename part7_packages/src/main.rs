/*
how to use mod from lib.rs
main.rs 必须把 lib.rs 当作一个外部依赖来引入。
*/

use part7_packages::part1::PI as PART1_PI;

fn foo() {
    println!("mod part1 PI is {PART1_PI}");
}

fn main() {
    hello_world::print("如何引用lib.rs中的mod", foo);
}
