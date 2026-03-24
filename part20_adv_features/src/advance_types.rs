/*
高级类型

*/

type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn use_type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

fn just_for_test() {
    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
        Box::new(|| println!("hi"))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn use_function_pointers() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");
}

pub fn foo() {
    hello_world::print("使用函数指针", use_function_pointers);
}
