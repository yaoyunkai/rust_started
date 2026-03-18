use num_traits::ToPrimitive;
use std::any::Any;
use std::any::type_name_of_val;

/* 
Rust 通过在编译时对泛型代码进行单态化（monomorphization）来实现这一点。
单态化就是把泛型代码转换成具体代码的过程，方法是用编译时实际用到的具体类型去填充泛型代码。

*/

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// std::cmp::PartialOrd
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn run_without_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

fn run_with_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<E: PartialOrd + Copy> Point<E> {
    pub fn get_max(&self) -> E {
        // Rust 编译器在处理 < 运算符时会自动进行借用
        if self.x < self.y { self.y } else { self.x }
    }
}

impl<T: ToPrimitive> Point<T> {
    pub fn distance_from_origin(&self) -> f64 {
        let x_f64 = self.x.to_f64().unwrap();
        let y_f64 = self.y.to_f64().unwrap();

        (x_f64.powi(2) + y_f64.powi(2)).sqrt()
    }
}

fn get_type_info_from_variables() {
    let point: Point<u32> = Point { x: 23, y: 45 };
    let point2 = Point { x: 4.0, y: 5.0 };

    println!("a 的类型: {}", type_name_of_val(&point)); // 输出: i32
    if point.type_id() == point2.type_id() {
        println!("point 和 point2 类型相同");
    } else {
        println!("point 和 point2 类型不相同");
    }

    let max = point.get_max();
    println!("max for point is {max}");

    let point3 = Point { x: 1, y: 0 };
    let dis = point3.distance_from_origin();
    println!("distance from origin is {dis:#?}");
}

fn check_function_enabled() {
    let point = Point {
        x: String::from("hello"),
        y: String::from("world"),
    };
    // point.distance_from_origin();

    println!(
        "can't call distance_from_origin for String type Point: {:?}",
        point
    );
}

pub fn foo() {
    hello_world::print("Run without Generics", run_without_generics);
    hello_world::print("Run with Generics", run_with_generics);
    hello_world::print("获取类型信息", get_type_info_from_variables);
    hello_world::print("String 类型的Point无法使用 distance_from_origin", check_function_enabled);
}
