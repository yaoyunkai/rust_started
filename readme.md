# Rust Started

预导入 preclude : https://doc.rust-lang.org/std/prelude/index.html  
标准库的 Prelude (std::prelude) 包含了 Rust 编程中最常用、最核心的项。

## 数据类型

![img.png](docs/img1.png)

## Trait 是什么

trait 是 Rust 的接口（类似其他语言的 interface / protocol）。 </br>
核心作用：
- 定义共享行为（方法签名）
- 实现多态
- 做泛型约束（where 子句 / impl Trait）

```rust
trait Drawable {
    fn draw(&self);
}

impl Drawable for Circle { ... }
impl Drawable for Square { ... }

```

## 属性

属性（attribute）是关于 Rust 代码片段的元数据；

## 生命周期标记 ???

## Box Rc RcCell

## 多线程中的 Channel, Mutex, Arc, 

> trait Send, Sync

## 异步

这些调用从一开始就不是阻塞的，并且我们只需定义程序想完成的一组任务，然后让运行时自行选择最佳的执行顺序和方式。

并行（parallelism）和并发（concurrency）的区别。<br/>
Rust 异步编程的关键元素是 futures 和 Rust 的 async 与 await 关键字。

## Others

语法糖展开（Desugaring） 和 语言项（Lang Items）。

`#[lang = "eq"]`

Rust 编译器（rustc）内部硬编码了一张表，规定了“当遇到 `*` 操作符时，去寻找标记为 `#[lang = "deref"]` 的特型”。

