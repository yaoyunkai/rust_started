#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

fn init_struct() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1 is {user1:#?}");
}

fn actions_with_struct_field() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.sign_in_count = 32;
    user1.username = String::from("Tom Peter");

    println!("user1 is {user1:#?}");
}

// 字段初始化简写语法
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 结构体更新语法
fn create_user() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("user1.active: {}", user1.active);
    // println!("user1.username: {}", user1.username);

    println!("user2 is {:#?}", user2);
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn use_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black is {black:#?}");
    println!("origin is {origin:#?}");

    let copy = black;
    println!("copy black is {copy:#?}");
}

// 类单元结构体（unit-like structs）
// 类单元结构体在你想要在某个类型上实现 trait，但又不需要在该类型本身中存储任何数据时会很有用
struct AlwaysEqual;

pub fn foo() {
    hello_world::print("结构体的初始化", init_struct);
    hello_world::print("结构体字段的使用", actions_with_struct_field);
    hello_world::print("结构体更新语法", create_user);
    hello_world::print("元组结构体的使用", use_tuple_struct);
}
