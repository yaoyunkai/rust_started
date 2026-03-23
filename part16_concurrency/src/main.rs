/*
Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送。

*/

mod use_channel;
mod use_mutex;
mod use_spawn;

fn main() {
    // println!("Hello, world!");
    // use_spawn::foo();
    // use_channel::foo();
    use_mutex::foo();
}
