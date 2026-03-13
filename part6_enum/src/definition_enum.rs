/*
枚举工作的另一个细节：每一个我们定义的枚举变体的名字也变成了一个构建枚举的实例的函数。

*/

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("执行了 Quit（退出）操作");
            }
            Message::Move { x, y } => {
                // 这里可以提取出结构体变体中的 x 和 y
                println!("执行了 Move 操作，移动到 x: {}, y: {}", x, y);
            }
            Message::Write(text) => {
                // 这里可以提取出元组变体中的 String
                // 注意：因为 self 是 &Message，所以这里的 text 类型是 &String
                println!("执行了 Write 操作，内容是: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                // 提取颜色值
                println!(
                    "执行了 ChangeColor 操作，颜色变更为 R:{}, G:{}, B:{}",
                    r, g, b
                );
            }
        }
    }
}

fn use_enum_value() {
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("the home     address is {home:#?}");
    println!("the loopback address is {loopback:#?}");

    let msg1 = Message::Write(String::from("Hello, world!"));
    let msg2 = Message::Move { x: 10, y: 20 };
    msg1.call();
    msg2.call();

    let value: Option<u32> = Option::from(None);

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: Option<i8> = Some(8);
    let y: Option<i8> = Some(66);

    let sum = x.unwrap() + y.unwrap();
    println!("sum is {sum}");
}

pub fn foo() {
    hello_world::print("enum的初始化", use_enum_value);
}
