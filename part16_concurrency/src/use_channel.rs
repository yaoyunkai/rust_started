/*
使用channel

线程间通信。

recv,
try_recv



*/
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn use_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        thread::sleep(Duration::from_secs(2));
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn use_multi_thread_send_message() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let to_call = |tx_inner: mpsc::Sender<String>| {
        let thread_id = std::thread::current().id();
        let datetime_formatted = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let message = format!("{:?} send at {}", thread_id, datetime_formatted);
        tx_inner.send(message).unwrap();
    };

    thread::spawn(move || {
        to_call(tx1);
    });
    thread::spawn(move || {
        to_call(tx2);
    });

    drop(tx);

    for received in rx {
        println!("主线程接收到: {}", received);
    }
}

pub fn foo() {
    hello_world::print("使用channel 发送消息", use_channel);
    hello_world::print("多个线程发送数据", use_multi_thread_send_message);
}
