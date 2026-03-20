/*
智能指针（smart pointers）是一类数据结构，它们的行为类似指针，但还拥有额外的元数据和功能。
引用只会借用数据，而智能指针在很多情况下会拥有它们所指向的数据。

Deref Drop trait

Box 用于在堆上分配值
Rc  一个引用计数类型，其数据可以有多个所有者
Ref  RefMut   它们是通过 RefCell<T> 访问的。而 RefCell<T> 是一个在运行时而非编译时执行借用规则的类型。

内部可变性（interior mutability）模式


*/
mod use_box;
mod use_drop_trait;
mod use_rc;

fn main() {
    // println!("Hello, world!");
    // use_box::foo();
    // use_drop_trait::foo();
    use_rc::foo();
}
