/*
使用spawn创建新的线程

*/
use std::thread;
use std::time::Duration;

fn use_spawn_create_thread() {
    let to_run = || {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    };

    let ret = thread::spawn(to_run);

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    ret.join().unwrap();
}

fn use_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

pub fn foo() {
    hello_world::print("使用spawn创建线程", use_spawn_create_thread);
    hello_world::print("使用move 转移所有权", use_move);
}
