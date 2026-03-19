fn call_once<F: FnOnce()>(f: F) {
    f();
}

fn run_with_fnonce() {
    let my_string = String::from("Hello");

    let consume_closure = || {
        let _moved_string = my_string;
        println!("字符串的所有权已被闭包拿走并消耗！");
    };

    call_once(consume_closure);

    // ❌ 如果你尝试再次调用 call_once(consume_closure); 编译器会报错
    // 因为 consume_closure 在第一次调用时就已经失效了
    // println!("{}", my_string);
}

fn call_mut<F: FnMut()>(mut f: F) {
    f();
    f(); // 允许调用多次
}

fn run_with_fnmut() {
    let mut count = 0;

    let mut increment_closure = || {
        count += 1; // 可变借用外部的 count
        println!("当前计数: {}", count);
    };

    call_mut(&mut increment_closure);

    // 闭包执行完毕并归还借用后，我们依然可以访问 count
    println!("最终计数: {}", count);
}

// 这个函数要求传入的闭包必须实现 Fn，它可以被无限次调用
fn call_fn<F: Fn()>(f: F) {
    f();
    f();
    f(); // 随便调用多少次都可以
}

fn run_with_fn() {
    // 结合当前环境信息创建一个字符串
    let greeting = String::from("Greetings from Ota City, JP on March 19, 2026!");

    let print_closure = || {
        // 仅仅是打印（不可变借用），不修改也不拿走所有权
        println!("不可变读取: {}", greeting);
    };

    call_fn(&print_closure);

    // 闭包执行完后，原变量完好无损，依然可以继续使用
    println!("闭包外部再次读取: {}", greeting);
}

pub fn foo() {
    hello_world::print("run with Fn", run_with_fn);
    hello_world::print("run with FnMut", run_with_fnmut);
    hello_world::print("run with FnOnce", run_with_fnonce);
}
