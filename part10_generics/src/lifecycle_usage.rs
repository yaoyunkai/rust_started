// fn demo1() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {r}");
// }

struct ImportantExcerpt<'a, T> {
    part: &'a str,
    data: T,
}

fn lifecycle_with_function() {
    let string1 = String::from("abcde");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn foo() {
    hello_world::print("生命周期的使用方式", lifecycle_with_function);
}
