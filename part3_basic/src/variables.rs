/*
变量默认是不可变的。

*/
// pub fn test_immutable() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

fn run_mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn variables_shadowing() {
    /*
    mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。

    */
    println!("start variables shadowing test");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// declare a const
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn runner() {
    println!("\n\nrun with mutable variables.");
    run_mutable();

    println!("\n\nrun variables_shadowing");
    variables_shadowing();

    println!("\n\n const variables");
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
}
