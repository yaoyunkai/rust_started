/*
让编译器帮你写 trait 的 impl 代码.
struct 只要被 drop，里面所有拥有所有权的字段（包括 String）都会被 drop。

*/
#[derive(Debug)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("drop was called: {}", self.0);
    }
}

fn usage() {
    let s = Data(String::from("hello"));
    takes_reference(&s);
    takes_ownership(s);
    // println!("{:?}", s);
    // takes_reference(&s);
}

fn takes_ownership(d: Data) {
    println!("接收到: {:?}", d);
    // 函数结束时 d drop
}

fn takes_reference(d: &Data) {
    println!("get reference of Data: {d:?}");
}

pub fn foo() {
    hello_world::print("Drop被调用的demo", usage);
}
