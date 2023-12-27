/*

定义一个enum


 */


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数。
#[derive(Debug)]
enum IpAddr1 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(self: &Self) {
        // 在这里定义方法体
        // self.call();
    }
}

pub fn run_define() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}, six: {:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("the loopback ip is {:#?}", loopback);
    println!("the home ip is {:#?}", home);


    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));
    println!("the loopback ip is {:#?}", loopback);
    println!("the home ip is {:#?}", home);


    let home = IpAddr2::V4(127, 0, 0, 1);
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("the loopback ip is {:#?}", loopback);
    println!("the home ip is {:#?}", home);
}

pub fn run_with_option() {
    let some_number = Option::Some(5);
    let some_char = Option::Some('e');

    let absent_number: Option<i32> = Option::None;

    println!("the some number is {:?}", some_number);
    println!("the some number is {:?}", some_char);
    println!("the some number is {:?}", absent_number);

    let x: i8 = 5;
    let y = Some(5);

    // different type
    let sum = x + y.expect("different type, can't add");

    println!("sum result is {:?}", sum);
}
