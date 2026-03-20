/*
Box<T>

递归类型（recursive type）的值可以拥有另一个同类型的值作为其自身的一部分。

Deref Drop

*/
use std::ops::Deref;

fn use_box() {
    let b = Box::new(89);
    println!("b is {b}");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn run_with_recursive() {
    use self::List::Cons;
    use self::List::Nil;

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// impl PartialEq<Box<i32>> for i32 {
//     fn eq(&self, other: &Box<i32>) -> bool {
//     }
// }

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn run_with_deref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let str1 = String::from("hello");
    let str2 = &str1;
    assert_eq!("hello", str1);
    assert_eq!("hello", str2);

    /*
    == 运算符（即 assert_eq! 底层使用的机制）依赖于 PartialEq 特型，
    而 Rust 编译器在进行特型匹配（Trait Resolution）时，不会自动触发解引用强制多态（Deref Coercion）。

    Deref 触发条件：
    函数或方法传参时：且目标参数的类型是明确已知的引用类型（如 &i32）。
    方法调用时：比如 z.some_method()，如果 Box 没有这个方法，编译器会自动解引用去 i32 上找。

    */
    let z = Box::from(x);
    assert_eq!(x, *z);
    // let new_num = *z + 1;
}

fn use_my_box() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    //  *(y.deref())
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // hello(&(*m)[..]);
}

fn hello(string: &str) {
    println!("hello, {string}");
}

pub fn foo() {
    hello_world::print("Use Box", use_box);
    hello_world::print("使用 Deref", run_with_deref);
    hello_world::print("使用 MyBox", use_my_box);
}
