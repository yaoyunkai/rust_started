/*

定义一个enum


 */


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

pub fn run_define() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);
}
