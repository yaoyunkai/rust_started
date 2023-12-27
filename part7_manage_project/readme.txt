一个包可以包含多个二进制 crate 项和一个可选的 crate 库

模块系统（the module system）
    包（Packages）：Cargo 的一个功能，它允许你构建、测试和分享 crate。
    Crates ：一个模块的树形结构，它形成了库或二进制项目。
    模块（Modules）和 use：允许你控制作用域和路径的私有性。
    路径（path）：一个命名例如结构体、函数或模块等项的方式
    
    
crate 有两种形式：二进制项和库。

crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块

包（package）是提供一系列功能的一个或者多个 crate。一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。
package被 Cargo 所管理

一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。
