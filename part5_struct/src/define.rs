/*

定义和实例化

元组结构体（tuple structs）
 
类单元结构体（unit-like structs）
类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。

可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）


*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


pub fn run_struct() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("user1 email address is {}", user1.email);

    // 结构体更新语法
    // 结构更新语法就像带有 = 的赋值，因为它移动了数据，
    // 所以 user1 的 username 被移动到了 user2
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2 username is {}", user2.username);
}

#[allow(dead_code)]
fn build_user1(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub struct Color(i32, i32, i32);

pub struct Point(i32, i32, i32);

struct AlwaysEqual;


pub fn test_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    // println!("the black is {black}");
}


// struct RefUser {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
