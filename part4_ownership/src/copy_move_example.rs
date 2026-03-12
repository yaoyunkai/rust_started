/*
Copy（复制） 和 Move（移动）

*/

#[derive(Debug)]
struct User {
    name: String, // // String 不支持 Copy，因此 User 也无法支持 Copy
}

#[derive(Debug, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn copy_move_usage() {
    println!("=== 场景一：基本类型的 Copy ===");
    let num1 = 100;
    let num2 = num1; // 触发 Copy：因为 i32 实现了 Copy 特型
    println!("num1 = {}, num2 = {}\n", num1, num2);

    println!("=== 场景二：复杂类型的 Move ===");
    let s1 = String::from("Hello");
    let s2 = s1; // 触发 Move：String 没有实现 Copy，所有权转移给 s2
    // println!("s1 = {}", s1);
    println!("s2 成功拿到了数据: {}\n", s2);

    println!("=== 场景三：自定义结构体的 Move (默认) ===");
    let user1 = User {
        name: String::from("Alice"),
    };
    let user2 = user1; // 触发 Move：user1 的所有权转移给了 user2
    // println!("user1: {:?}", user1);
    println!("user2 成功拿到了数据: {:?}\n", user2);

    println!("=== 场景四：自定义结构体的 Copy ===");
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // 触发 Copy：因为我们为 Point 派生了 Copy 特型
    println!("p1 = {:?}, p2 = {:?}", p1, p2);
}

pub fn foo() {
    hello_world::print("Copy Move的示例", copy_move_usage);
}
