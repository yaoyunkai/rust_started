/*
panic usage
*/

fn panic_usage() {
    panic!("crash and burn");
}

pub fn foo() {
    hello_world::print("panic 的使用方式", panic_usage);
}
