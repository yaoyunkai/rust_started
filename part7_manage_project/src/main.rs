/*

内联模块不用 pub 也可以被看到

一个模块里的代码默认对其父模块私有。

绝对路径（absolute path）是以 crate 根（root）开头的全路径；
对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于当前 crate 的代码，则以字面值 crate 开头。

相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。


我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。


as 关键字

use std::fmt::Result;
use std::io::Result as IoResult;

将一个路径下 所有 公有项引入作用域，可以指定路径后跟 *
use std::collections::*;

 */


// 1. 内联模块
mod mod1 {
    pub const PI: f64 = 3.14;
}


// 2. 在文件 src/garden.rs
mod mod2;

// 3. 在文件 src/garden/mod.rs
mod mod3;


mod mod_abc {
    fn run_5() {
        let num = self::mod_abc_456::SIMPLE_1;
        println!("the num is {num}");
    }

    mod mod_abc_123 {
        pub fn run_1() {}

        fn run_2() {}
    }

    mod mod_abc_456 {
        pub const SIMPLE_1: u32 = 123;

        fn run_3() {
            run_4();
            super::run_5();
            super::mod_abc_123::run_1();
        }

        fn run_4() {
            self::run_3();
        }
    }
}


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

fn main() {
    println!("Hello, world!");
    println!("the PI from mod1 is {}", crate::mod1::PI);
    println!("the one hour in seconds is {}", crate::mod2::ONE_HOUR_IN_SECONDS);

    crate::mod2::mod2_mod1::print_mod_name();

    eat_at_restaurant();
}
