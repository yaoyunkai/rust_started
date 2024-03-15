/*

生命周期

借用检查器 borrow checker

生命周期注解并不改变任何引用的生命周期的长短。
相反它们描述了多个引用生命周期相互的关系，而不影响其生命周期。


目前为止，我们定义的结构体全都包含拥有所有权的类型。
也可以定义包含引用的结构体，不过这需要为结构体定义中的每一个引用添加生命周期注解


生命周期省略规则（lifetime elision rules）

这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
所有的字符串字面值都拥有 'static 生命周期，

 */



pub fn test_lifetime() {
    let r: u32 = 0;

    {
        let x = 5;
        // r = &x;
    }

    println!("r: {}", r);
}


pub fn test_lifetime2() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}


// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}


pub fn test_lifetime3() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
