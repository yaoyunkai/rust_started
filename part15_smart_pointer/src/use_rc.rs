/*
Rc<T>

*/

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn run_with_rc() {
    use self::List::{Cons, Nil};

    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
}

fn check_reference_count() {
    use self::List::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

pub fn foo() {
    hello_world::print("Run with Rc<T>", run_with_rc);
    hello_world::print("检查 Rc的引用计数", check_reference_count);
}
