/*
可恢复的（recoverable）和 不可恢复的（unrecoverable）错误

 */
mod panic_usage;

mod result_usage;

fn main() {
    // println!("Hello, world!");
    // panic_usage::test_panic();

    result_usage::run_result_with_unwrap();
}
