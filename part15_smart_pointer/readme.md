# 智能指针

## 处理可变引用的 Deref 强制转换

Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：

1. 当 `T: Deref<Target=U>` 时从 `&T` 到 `&U`。
1. 当 `T: DerefMut<Target=U>` 时从 `&mut T` 到 `&mut U`。
1. 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U`。

## Drop Trait

## `Rc<T>`

通过不可变引用， `Rc<T>` 允许在程序的多个部分之间只读地共享数据。

## `RefCell<T>` 和内部可变性模式

