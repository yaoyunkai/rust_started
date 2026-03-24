/*
解引用裸指针

什么时候需要unsafe

使用裸指针借用操作符（raw borrow operators）创建裸指针

*/

fn use_raw_pointers() {
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
unsafe fn dangerous() {}

pub fn foo() {
    unsafe {
        dangerous();
    }
    // dangerous();
    hello_world::print("使用裸指针", use_raw_pointers);
}
