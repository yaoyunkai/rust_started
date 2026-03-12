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

pub fn foo() {
    hello_world::print("结构体的初始化", init_struct);
    hello_world::print("结构体字段的使用", actions_with_struct_field)
}
