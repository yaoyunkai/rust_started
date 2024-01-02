/*



 */
use std::fmt::{Debug, Display};

pub trait Summary {
    // fn summarize(self: &Self) -> String {
    //     String::from("(Read more...)")
    // }

    fn summarize(self: &Self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Summary for Vec<u64> {
    fn summarize(self: &Self) -> String {
        todo!();
    }
}

// trait 作为参数
#[allow(unused)]
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound 语法
#[allow(unused)]
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news Aha! {}", item.summarize());
}

#[allow(unused)]
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    // 这适用于 item1 和 item2 允许是不同类型的情况（只要它们都实现了 Summary）。
}

#[allow(unused)]
pub fn notify3<T: Summary>(item1: &T, item2: &T) {
    // 泛型 T 被指定为 item1 和 item2 的参数限制，如此传递给参数 item1 和 item2 值的具体类型必须一致。
}


#[allow(unused)]
pub fn notify4(item: &(impl Summary + Display)) {
    // 
}

#[allow(unused)]
pub fn notify5<T: Summary + Display>(item: &T) {
    // 
}

#[allow(unused)]
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

#[allow(unused)]
fn some_function1<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone, U: Clone + Debug,
{
    0
}


#[allow(unused)]
pub fn run_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
}
