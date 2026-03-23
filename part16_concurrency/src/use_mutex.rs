/*
使用互斥锁

*/
use std::sync::{Arc, Mutex};
use std::thread;

fn use_mutex_in_single_thread() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    print!("m = {m:?}");
}

fn use_mutex_in_multi_thread() {
    let shared_list = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for i in 0..5 {
        let list_clone = Arc::clone(&shared_list);
        let handle = thread::spawn(move || {
            let mut list = list_clone.lock().unwrap();
            list.push(i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let final_list = shared_list.lock().unwrap();
    println!("所有线程执行完毕。最终的列表内容: {:?}", *final_list);
}

pub fn foo() {
    hello_world::print("使用mutex", use_mutex_in_single_thread);
    hello_world::print("多线程使用mutex", use_mutex_in_multi_thread);
}
