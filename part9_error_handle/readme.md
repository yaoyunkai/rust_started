# ? 运算符

核心作用：提前返回错误，简化错误处理。

```rust
fn example() -> Result<i32, Error> {
    let a = func1()?;        // 成功 → 继续，失败 → 直接返回 Err
    let b = func2()?;
    Ok(a + b)
}

```

相当于

```rust
let a = match func1() {
    Ok(v)  => v,
    Err(e) => return Err(e),
};

```

适用范围：
- `Result<T, E>`（最常见）
- `Option<T>`（返回 None 时提前退出）
- 实现了 `std::ops::Try` 的类型（夜间版可自定义）

被 ? 作用的错误值会经过 from 函数。这个函数定义在标准库的 `From trait` 中， 用于把一种类型的值转换成另一种类型。
当 ? 运算符调用 from 函数时，接收到的错误类型会被转换成当前函数返回类型里定义的错误类型。

from 函数 是 `std::convert::From trait` 里的唯一方法

为什么要经过 From => 让 `?` 能自动把内层错误类型 → 当前函数返回的错误类型。

----

**From trait 的其他主要作用**（除了 ? 的错误转换）：

- **构造实例**：`String::from("hello")`、`Vec::from([1,2,3])`、`PathBuf::from("/tmp")`
- **类型转换的统一接口**：让代码更泛型、可重用（e.g. `T: From<U>`）
- **自动获得 Into**：实现 From<A> for B 后，A 自动获得 .into() → B（最常见用法之一）
- **集合/容器初始化**：`HashMap::from([...])`、`Cow::from(...)` 等
- **标准库中大量转换**：`&str → String`、`&[u8] → Vec<u8>`、`i32 → f64` 等
- **自定义类型间转换**：定义清晰、无歧义的转换路径

