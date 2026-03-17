# 包、Crates 与模块

- 包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
- Crates：一个模块树，可以产生一个库或可执行文件。
- 模块（Modules）和 use：允许你控制作用域和路径的私有性。
- 路径（path）：一个为例如结构体、函数或模块等项命名的方式。

crate 有两种形式：二进制 crate 和库 crate。二进制 crate（Binary crates）可以被编译为可执行程序，比如命令行程序或者服务端。
它们必须有一个名为 main 函数来定义当程序被执行的时候所需要做的事情。目前我们所创建的 crate 都是二进制 crate。

包（package）是提供一系列功能的一个或者多个 crate 的捆绑。一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。

包中可以包含至多一个库 crate(library crate)。
包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。

## 模块的定义方式

声明模块: 在 crate 根文件中，你可以声明一个新模块；比如，用 mod garden; 
声明了一个叫做 garden 的模块。编译器会在下列路径中寻找模块代码：
- 内联，用大括号替换 mod garden 后跟的分号
- 在文件 src/garden.rs
- 在文件 src/garden/mod.rs

在 main.rs 中声明的 mod a; 和 mod b; 都处于项目的根层级（Root）。
根据 Rust 的可见性规则，根层级的模块对整个项目内部都是可见的。
模块 b 总是可以通过绝对路径 crate::a 来访问模块 a

