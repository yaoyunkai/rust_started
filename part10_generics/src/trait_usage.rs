/*
Trait 类似于 interface

在 Rust 中，如果你将一个 trait 声明为 pub，那么它内部定义的所有方法（以及关联类型、常量）自动就是公开的。

覆盖实现（Blanket Implementation）。

只有当你想做“只要实现了 A，就自动实现 B”这种高级的、
批量的 trait 绑定时，你才需要写类似 <T: TraitA> TraitB for T 的代码。

*/
use std::fmt;
use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;

    fn others(self: &Self) {
        println!("call Summary.others, then call summarize");
        println!("summary: {}", self.summarize());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("number: {}", self.to_string())
    }
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn use_trait() {
    let art1 = NewsArticle {
        headline: String::from("ABC"),
        location: "NewYork".to_string(),
        author: "Tom".to_string(),
        content: String::from("Converts to this type from the input type."),
    };

    println!("{}", art1.summarize());

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
    post.others();

    let num = 32_i32;
    println!("{}", num.summarize());
}

// 使用 trait作为参数
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound 语法
fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    0
}

fn some_function_update<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// 返回trait
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

fn use_trait_as_params() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    notify(&post);
    notify1(&post);
}

fn trait_as_return_type() {
    let obj = returns_summarizable();
    obj.others();
}

struct User {
    username: String,
    age: u8,
}

// 实现 Display trait
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 定义如何将 User 格式化为字符串
        write!(f, "User: {} (Age: {})", self.username, self.age)
    }
}

fn use_display() {
    let my_user = User {
        username: String::from("hello world"),
        age: 2,
    };

    // 魔法发生：因为你实现了 Display，你的 struct 自动拥有了 .to_string() 方法！
    let user_string = my_user.to_string();

    println!("转换后的字符串是: {}", user_string);

    // 附赠好处：你也可以直接用 {} 打印它了
    println!("直接打印: {}", my_user);
}

// 1. 定义“有名字”的 Trait
trait HasName {
    fn get_name(&self) -> &str;
}

// 2. 定义“打招呼”的 Trait
trait Greet {
    fn greet(&self);
}

// 3. 这就是你需要写类似代码的时候！
// 意思是：对于任何实现了 HasName 的类型 T，我自动为你实现 Greet
impl<T: HasName> Greet for T {
    fn greet(&self) {
        println!("你好，我是 {}！", self.get_name());
    }
}

pub fn foo() {
    hello_world::print("使用Trait", use_trait);
    hello_world::print("使用trait作为参数", use_trait_as_params);
    hello_world::print("Trait作为返回类型", trait_as_return_type);
    hello_world::print("使用Display Trait", use_display);
}
