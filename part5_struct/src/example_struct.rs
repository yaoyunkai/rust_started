#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
// 方法调用是 Rust 中少数几个拥有这种行为的地方。
// 所有在 impl 块中定义的函数被称为 关联函数（associated functions），
// 因为它们与 impl 后面命名的类型相关。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl From<u32> for Rectangle {
    /// create a square
    ///
    /// # Arguments
    ///
    /// * `value`:
    ///
    /// returns: Rectangle
    fn from(value: u32) -> Self {
        Rectangle {
            width: value,
            height: value,
        }
    }
}

// trait Box {
//     fn from(value: u32) -> Self;
// }
//
// impl Box for Rectangle {
//     fn from(value: u32) -> Self {
//         Rectangle {
//             width: value,
//             height: value,
//         }
//     }
// }

fn use_struct_function() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // let rect2 = Box::from(4);

    let rect2 = Rectangle::square(5);
    println!("the area of rect2 is {}", rect2.area());

    let rect3 = Rectangle::from(5);
    println!("the area of rect3 is {}", rect3.area());
}

pub fn foo() {
    hello_world::print("使用结构体方法", use_struct_function);
}
